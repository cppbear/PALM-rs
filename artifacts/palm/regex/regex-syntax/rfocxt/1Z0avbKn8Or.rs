pub type Result<T> = result::Result<T, Error>;
use std::cmp;
use std::error;
use std::fmt;
use std::result;
use ast;
use hir;
#[derive(Debug)]
pub struct Formatter<'e, E: 'e> {
    /// The original regex pattern in which the error occurred.
    pattern: &'e str,
    /// The error kind. It must impl fmt::Display.
    err: &'e E,
    /// The primary span of the error.
    span: &'e ast::Span,
    /// An auxiliary and optional span, in case the error needs to point to
    /// two locations (e.g., when reporting a duplicate capture group name).
    aux_span: Option<&'e ast::Span>,
}
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
impl<'e, E: fmt::Display> fmt::Display for Formatter<'e, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spans = Spans::from_formatter(self);
        if self.pattern.contains('\n') {
            let divider = repeat_char('~', 79);
            writeln!(f, "regex parse error:")?;
            writeln!(f, "{}", divider)?;
            let notated = spans.notate();
            write!(f, "{}", notated)?;
            writeln!(f, "{}", divider)?;
            if !spans.multi_line.is_empty() {
                let mut notes = vec![];
                for span in &spans.multi_line {
                    notes
                        .push(
                            format!(
                                "on line {} (column {}) through line {} (column {})", span
                                .start.line, span.start.column, span.end.line, span.end
                                .column - 1
                            ),
                        );
                }
                writeln!(f, "{}", notes.join("\n"))?;
            }
            write!(f, "error: {}", self.err)?;
        } else {
            writeln!(f, "regex parse error:")?;
            let notated = Spans::from_formatter(self).notate();
            write!(f, "{}", notated)?;
            write!(f, "error: {}", self.err)?;
        }
        Ok(())
    }
}
impl<'p> Spans<'p> {
    fn from_formatter<'e, E: fmt::Display>(fmter: &'p Formatter<'e, E>) -> Spans<'p> {}
    fn add(&mut self, span: ast::Span) {}
    fn notate(&self) -> String {
        let mut notated = String::new();
        for (i, line) in self.pattern.lines().enumerate() {
            if self.line_number_width > 0 {
                notated.push_str(&self.left_pad_line_number(i + 1));
                notated.push_str(": ");
            } else {
                notated.push_str("    ");
            }
            notated.push_str(line);
            notated.push('\n');
            if let Some(notes) = self.notate_line(i) {
                notated.push_str(&notes);
                notated.push('\n');
            }
        }
        notated
    }
    fn notate_line(&self, i: usize) -> Option<String> {}
    fn left_pad_line_number(&self, n: usize) -> String {}
    fn line_number_padding(&self) -> usize {}
}
fn repeat_char(c: char, count: usize) -> String {
    ::std::iter::repeat(c).take(count).collect()
}
