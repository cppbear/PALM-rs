use std::cmp::Ordering;
use std::error;
use std::fmt;
pub use ast::visitor::{Visitor, visit};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassSetRange {
    /// The span of this range.
    pub span: Span,
    /// The start of this range.
    pub start: Literal,
    /// The end of this range.
    pub end: Literal,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
}
#[derive(Clone, Eq, Ord)]
pub struct Literal {
    v: Vec<u8>,
    cut: bool,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    /// A single character represented by a Unicode scalar value.
    Unicode(char),
    /// A single character represented by an arbitrary byte.
    Byte(u8),
}
impl ClassSetRange {
    pub fn is_valid(&self) -> bool {
        self.start.c <= self.end.c
    }
}
