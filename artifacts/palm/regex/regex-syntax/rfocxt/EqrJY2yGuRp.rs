use std::fmt;
use ast::{self, Ast};
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassSetItem {
    /// An empty item.
    ///
    /// Note that a bracketed character class cannot contain a single empty
    /// item. Empty items can appear when using one of the binary operators.
    /// For example, `[&&]` is the intersection of two empty classes.
    Empty(Span),
    /// A single literal.
    Literal(Literal),
    /// A range between two literals.
    Range(ClassSetRange),
    /// An ASCII character class, e.g., `[:alnum:]` or `[:punct:]`.
    Ascii(ClassAscii),
    /// A Unicode character class, e.g., `\pL` or `\p{Greek}`.
    Unicode(ClassUnicode),
    /// A perl character class, e.g., `\d` or `\W`.
    Perl(ClassPerl),
    /// A bracketed character class set, which may contain zero or more
    /// character ranges and/or zero or more nested classes. e.g.,
    /// `[a-zA-Z\pL]`.
    Bracketed(Box<ClassBracketed>),
    /// A union of items.
    Union(ClassSetUnion),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassSet {
    /// An item, which can be a single literal, range, nested character class
    /// or a union of items.
    Item(ClassSetItem),
    /// A single binary operation (i.e., &&, -- or ~~).
    BinaryOp(ClassSetBinaryOp),
}
impl<'a> fmt::Debug for ClassFrame<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            ClassFrame::Union { .. } => "Union",
            ClassFrame::Binary { .. } => "Binary",
            ClassFrame::BinaryLHS { .. } => "BinaryLHS",
            ClassFrame::BinaryRHS { .. } => "BinaryRHS",
        };
        write!(f, "{}", x)
    }
}
