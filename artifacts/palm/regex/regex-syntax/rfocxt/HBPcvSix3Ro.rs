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
enum ClassInduct<'a> {
    Item(&'a ast::ClassSetItem),
    BinaryOp(&'a ast::ClassSetBinaryOp),
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
impl<'a> fmt::Debug for ClassInduct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            ClassInduct::Item(it) => {
                match *it {
                    ast::ClassSetItem::Empty(_) => "Item(Empty)",
                    ast::ClassSetItem::Literal(_) => "Item(Literal)",
                    ast::ClassSetItem::Range(_) => "Item(Range)",
                    ast::ClassSetItem::Ascii(_) => "Item(Ascii)",
                    ast::ClassSetItem::Perl(_) => "Item(Perl)",
                    ast::ClassSetItem::Unicode(_) => "Item(Unicode)",
                    ast::ClassSetItem::Bracketed(_) => "Item(Bracketed)",
                    ast::ClassSetItem::Union(_) => "Item(Union)",
                }
            }
            ClassInduct::BinaryOp(it) => {
                match it.kind {
                    ast::ClassSetBinaryOpKind::Intersection => "BinaryOp(Intersection)",
                    ast::ClassSetBinaryOpKind::Difference => "BinaryOp(Difference)",
                    ast::ClassSetBinaryOpKind::SymmetricDifference => {
                        "BinaryOp(SymmetricDifference)"
                    }
                }
            }
        };
        write!(f, "{}", x)
    }
}
