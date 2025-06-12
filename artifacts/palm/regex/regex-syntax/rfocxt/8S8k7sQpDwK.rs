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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBracketed {
    /// The span of this class.
    pub span: Span,
    /// Whether this class is negated or not. e.g., `[a]` is not negated but
    /// `[^a]` is.
    pub negated: bool,
    /// The type of this set. A set is either a normal union of things, e.g.,
    /// `[abc]` or a result of applying set operations, e.g., `[\pL--c]`.
    pub kind: ClassSet,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassSetBinaryOp {
    /// The span of this operation. e.g., the `a-z--[h-p]` in `[a-z--h-p]`.
    pub span: Span,
    /// The type of this set operation.
    pub kind: ClassSetBinaryOpKind,
    /// The left hand side of the operation.
    pub lhs: Box<ClassSet>,
    /// The right hand side of the operation.
    pub rhs: Box<ClassSet>,
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
impl<'a> HeapVisitor<'a> {
    fn new() -> HeapVisitor<'a> {}
    fn visit<V: Visitor>(
        &mut self,
        mut ast: &'a Ast,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {}
    fn induct<V: Visitor>(
        &mut self,
        ast: &'a Ast,
        visitor: &mut V,
    ) -> Result<Option<Frame<'a>>, V::Err> {}
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {}
    fn visit_class<V: Visitor>(
        &mut self,
        ast: &'a ast::ClassBracketed,
        visitor: &mut V,
    ) -> Result<(), V::Err> {
        let mut ast = ClassInduct::from_bracketed(ast);
        loop {
            self.visit_class_pre(&ast, visitor)?;
            if let Some(x) = self.induct_class(&ast) {
                let child = x.child();
                self.stack_class.push((ast, x));
                ast = child;
                continue;
            }
            self.visit_class_post(&ast, visitor)?;
            loop {
                let (post_ast, frame) = match self.stack_class.pop() {
                    None => return Ok(()),
                    Some((post_ast, frame)) => (post_ast, frame),
                };
                if let Some(x) = self.pop_class(frame) {
                    if let ClassFrame::BinaryRHS { ref op, .. } = x {
                        visitor.visit_class_set_binary_op_in(op)?;
                    }
                    ast = x.child();
                    self.stack_class.push((post_ast, x));
                    break;
                }
                self.visit_class_post(&post_ast, visitor)?;
            }
        }
    }
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
    fn induct_class(&self, ast: &ClassInduct<'a>) -> Option<ClassFrame<'a>> {
        match *ast {
            ClassInduct::Item(&ast::ClassSetItem::Bracketed(ref x)) => {
                match x.kind {
                    ast::ClassSet::Item(ref item) => {
                        Some(ClassFrame::Union {
                            head: item,
                            tail: &[],
                        })
                    }
                    ast::ClassSet::BinaryOp(ref op) => {
                        Some(ClassFrame::Binary { op: op })
                    }
                }
            }
            ClassInduct::Item(&ast::ClassSetItem::Union(ref x)) => {
                if x.items.is_empty() {
                    None
                } else {
                    Some(ClassFrame::Union {
                        head: &x.items[0],
                        tail: &x.items[1..],
                    })
                }
            }
            ClassInduct::BinaryOp(op) => {
                Some(ClassFrame::BinaryLHS {
                    op: op,
                    lhs: &op.lhs,
                    rhs: &op.rhs,
                })
            }
            _ => None,
        }
    }
    fn pop_class(&self, induct: ClassFrame<'a>) -> Option<ClassFrame<'a>> {
        match induct {
            ClassFrame::Union { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(ClassFrame::Union {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
            ClassFrame::Binary { .. } => None,
            ClassFrame::BinaryLHS { op, rhs, .. } => {
                Some(ClassFrame::BinaryRHS {
                    op: op,
                    rhs: rhs,
                })
            }
            ClassFrame::BinaryRHS { .. } => None,
        }
    }
}
impl<'a> ClassFrame<'a> {
    fn child(&self) -> ClassInduct<'a> {
        match *self {
            ClassFrame::Union { head, .. } => ClassInduct::Item(head),
            ClassFrame::Binary { op, .. } => ClassInduct::BinaryOp(op),
            ClassFrame::BinaryLHS { ref lhs, .. } => ClassInduct::from_set(lhs),
            ClassFrame::BinaryRHS { ref rhs, .. } => ClassInduct::from_set(rhs),
        }
    }
}
impl<'a> ClassInduct<'a> {
    fn from_bracketed(ast: &'a ast::ClassBracketed) -> ClassInduct<'a> {
        ClassInduct::from_set(&ast.kind)
    }
    fn from_set(ast: &'a ast::ClassSet) -> ClassInduct<'a> {}
}
