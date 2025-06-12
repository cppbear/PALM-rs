use std::cmp::Ordering;
use std::error;
use std::fmt;
pub use ast::visitor::{Visitor, visit};
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position {
    /// The absolute offset of this position, starting at `0` from the
    /// beginning of the regular expression pattern string.
    pub offset: usize,
    /// The line number, starting at `1`.
    pub line: usize,
    /// The approximate column number, starting at `1`.
    pub column: usize,
}
impl Span {
    pub fn new(start: Position, end: Position) -> Span {}
    pub fn splat(pos: Position) -> Span {}
    pub fn with_start(self, pos: Position) -> Span {
        Span { start: pos, ..self }
    }
    pub fn with_end(self, pos: Position) -> Span {}
    pub fn is_one_line(&self) -> bool {}
    pub fn is_empty(&self) -> bool {}
}
