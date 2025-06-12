use std::fmt;
use ast::{self, Ast};
struct HeapVisitor<'a> {
    /// A stack of `Ast` nodes. This is roughly analogous to the call stack
    /// used in a typical recursive visitor.
    stack: Vec<(&'a Ast, Frame<'a>)>,
    /// Similar to the `Ast` stack above, but is used only for character
    /// classes. In particular, character classes embed their own mini
    /// recursive syntax.
    stack_class: Vec<(ClassInduct<'a>, ClassFrame<'a>)>,
}
enum Frame<'a> {
    /// A stack frame allocated just before descending into a repetition
    /// operator's child node.
    Repetition(&'a ast::Repetition),
    /// A stack frame allocated just before descending into a group's child
    /// node.
    Group(&'a ast::Group),
    /// The stack frame used while visiting every child node of a concatenation
    /// of expressions.
    Concat {
        /// The child node we are currently visiting.
        head: &'a Ast,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Ast],
    },
    /// The stack frame used while visiting every child node of an alternation
    /// of expressions.
    Alternation {
        /// The child node we are currently visiting.
        head: &'a Ast,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Ast],
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ast {
    /// An empty regex that matches everything.
    Empty(Span),
    /// A set of flags, e.g., `(?is)`.
    Flags(SetFlags),
    /// A single character literal, which includes escape sequences.
    Literal(Literal),
    /// The "any character" class.
    Dot(Span),
    /// A single zero-width assertion.
    Assertion(Assertion),
    /// A single character class. This includes all forms of character classes
    /// except for `.`. e.g., `\d`, `\pN`, `[a-z]` and `[[:alpha:]]`.
    Class(Class),
    /// A repetition operator applied to an arbitrary regular expression.
    Repetition(Repetition),
    /// A grouped regular expression.
    Group(Group),
    /// An alternation of regular expressions.
    Alternation(Alternation),
    /// A concatenation of regular expressions.
    Concat(Concat),
}
enum Frame<'a> {
    /// A stack frame allocated just before descending into a repetition
    /// operator's child node.
    Repetition(&'a hir::Repetition),
    /// A stack frame allocated just before descending into a group's child
    /// node.
    Group(&'a hir::Group),
    /// The stack frame used while visiting every child node of a concatenation
    /// of expressions.
    Concat {
        /// The child node we are currently visiting.
        head: &'a Hir,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Hir],
    },
    /// The stack frame used while visiting every child node of an alternation
    /// of expressions.
    Alternation {
        /// The child node we are currently visiting.
        head: &'a Hir,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Hir],
    },
}
enum ClassFrame<'a> {
    /// The stack frame used while visiting every child node of a union of
    /// character class items.
    Union {
        /// The child node we are currently visiting.
        head: &'a ast::ClassSetItem,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [ast::ClassSetItem],
    },
    /// The stack frame used while a binary class operation.
    Binary { op: &'a ast::ClassSetBinaryOp },
    /// A stack frame allocated just before descending into a binary operator's
    /// left hand child node.
    BinaryLHS {
        op: &'a ast::ClassSetBinaryOp,
        lhs: &'a ast::ClassSet,
        rhs: &'a ast::ClassSet,
    },
    /// A stack frame allocated just before descending into a binary operator's
    /// right hand child node.
    BinaryRHS { op: &'a ast::ClassSetBinaryOp, rhs: &'a ast::ClassSet },
}
enum ClassInduct<'a> {
    Item(&'a ast::ClassSetItem),
    BinaryOp(&'a ast::ClassSetBinaryOp),
}
impl<'a> HeapVisitor<'a> {
    fn new() -> HeapVisitor<'a> {}
    fn visit<V: Visitor>(
        &mut self,
        mut ast: &'a Ast,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {
        self.stack.clear();
        self.stack_class.clear();
        visitor.start();
        loop {
            visitor.visit_pre(ast)?;
            if let Some(x) = self.induct(ast, &mut visitor)? {
                let child = x.child();
                self.stack.push((ast, x));
                ast = child;
                continue;
            }
            visitor.visit_post(ast)?;
            loop {
                let (post_ast, frame) = match self.stack.pop() {
                    None => return visitor.finish(),
                    Some((post_ast, frame)) => (post_ast, frame),
                };
                if let Some(x) = self.pop(frame) {
                    if let Frame::Alternation { .. } = x {
                        visitor.visit_alternation_in()?;
                    }
                    ast = x.child();
                    self.stack.push((post_ast, x));
                    break;
                }
                visitor.visit_post(post_ast)?;
            }
        }
    }
    fn induct<V: Visitor>(
        &mut self,
        ast: &'a Ast,
        visitor: &mut V,
    ) -> Result<Option<Frame<'a>>, V::Err> {}
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {
        match induct {
            Frame::Repetition(_) => None,
            Frame::Group(_) => None,
            Frame::Concat { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(Frame::Concat {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
            Frame::Alternation { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(Frame::Alternation {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
        }
    }
    fn visit_class<V: Visitor>(
        &mut self,
        ast: &'a ast::ClassBracketed,
        visitor: &mut V,
    ) -> Result<(), V::Err> {}
    fn visit_class_pre<V: Visitor>(
        &self,
        ast: &ClassInduct<'a>,
        visitor: &mut V,
    ) -> Result<(), V::Err> {}
    fn visit_class_post<V: Visitor>(
        &self,
        ast: &ClassInduct<'a>,
        visitor: &mut V,
    ) -> Result<(), V::Err> {}
    fn induct_class(&self, ast: &ClassInduct<'a>) -> Option<ClassFrame<'a>> {}
    fn pop_class(&self, induct: ClassFrame<'a>) -> Option<ClassFrame<'a>> {}
}
impl<'a> Frame<'a> {
    fn child(&self) -> &'a Ast {
        match *self {
            Frame::Repetition(rep) => &rep.ast,
            Frame::Group(group) => &group.ast,
            Frame::Concat { head, .. } => head,
            Frame::Alternation { head, .. } => head,
        }
    }
}
