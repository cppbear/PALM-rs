pub type Result<T> = result::Result<T, Error>;
use std::cmp;
use std::error;
use std::fmt;
use std::result;
use ast;
use hir;
struct Spans<'p> {
    /// The original regex pattern string.
    pattern: &'p str,
    /// The total width that should be used for line numbers. The width is
    /// used for left padding the line numbers for alignment.
    ///
    /// A value of `0` means line numbers should not be displayed. That is,
    /// the pattern is itself only one line.
    line_number_width: usize,
    /// All error spans that occur on a single line. This sequence always has
    /// length equivalent to the number of lines in `pattern`, where the index
    /// of the sequence represents a line number, starting at `0`. The spans
    /// in each line are sorted in ascending order.
    by_line: Vec<Vec<ast::Span>>,
    /// All error spans that occur over one or more lines. That is, the start
    /// and end position of the span have different line numbers. The spans are
    /// sorted in ascending order.
    multi_line: Vec<ast::Span>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
impl<'p> Spans<'p> {
    fn from_formatter<'e, E: fmt::Display>(fmter: &'p Formatter<'e, E>) -> Spans<'p> {}
    fn add(&mut self, span: ast::Span) {}
    fn notate(&self) -> String {}
    fn notate_line(&self, i: usize) -> Option<String> {}
    fn left_pad_line_number(&self, n: usize) -> String {}
    fn line_number_padding(&self) -> usize {
        if self.line_number_width == 0 { 4 } else { 2 + self.line_number_width }
    }
}
