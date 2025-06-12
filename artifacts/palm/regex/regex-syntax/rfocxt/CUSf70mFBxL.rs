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
pub struct Alternation {
    /// The span of this alternation.
    pub span: Span,
    /// The alternate regular expressions.
    pub asts: Vec<Ast>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Concat {
    /// The span of this concatenation.
    pub span: Span,
    /// The concatenation regular expressions.
    pub asts: Vec<Ast>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    /// The span of this operation.
    pub span: Span,
    /// The actual operation.
    pub op: RepetitionOp,
    /// Whether this operation was applied greedily or not.
    pub greedy: bool,
    /// The regular expression under repetition.
    pub ast: Box<Ast>,
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
pub struct Group {
    /// The span of this group.
    pub span: Span,
    /// The kind of this group.
    pub kind: GroupKind,
    /// The regular expression in this group.
    pub ast: Box<Ast>,
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
pub enum Class {
    /// A Unicode character class, e.g., `\pL` or `\p{Greek}`.
    Unicode(ClassUnicode),
    /// A perl character class, e.g., `\d` or `\W`.
    Perl(ClassPerl),
    /// A bracketed character class set, which may contain zero or more
    /// character ranges and/or zero or more nested classes. e.g.,
    /// `[a-zA-Z\pL]`.
    Bracketed(ClassBracketed),
}
enum ClassInduct<'a> {
    Item(&'a ast::ClassSetItem),
    BinaryOp(&'a ast::ClassSetBinaryOp),
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
    ) -> Result<Option<Frame<'a>>, V::Err> {
        Ok(
            match *ast {
                Ast::Class(ast::Class::Bracketed(ref x)) => {
                    self.visit_class(x, visitor)?;
                    None
                }
                Ast::Repetition(ref x) => Some(Frame::Repetition(x)),
                Ast::Group(ref x) => Some(Frame::Group(x)),
                Ast::Concat(ref x) if x.asts.is_empty() => None,
                Ast::Concat(ref x) => {
                    Some(Frame::Concat {
                        head: &x.asts[0],
                        tail: &x.asts[1..],
                    })
                }
                Ast::Alternation(ref x) if x.asts.is_empty() => None,
                Ast::Alternation(ref x) => {
                    Some(Frame::Alternation {
                        head: &x.asts[0],
                        tail: &x.asts[1..],
                    })
                }
                _ => None,
            },
        )
    }
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {}
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
