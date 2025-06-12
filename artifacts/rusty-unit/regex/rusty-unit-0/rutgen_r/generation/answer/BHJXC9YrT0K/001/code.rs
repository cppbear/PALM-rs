// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

struct Error {
    span: Span,
}

impl Error {
    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[test]
fn test_span() {
    let error = Error {
        span: Span { start: 0, end: 5 },
    };
    let result = error.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 5);
}

#[test]
fn test_span_with_other_values() {
    let error = Error {
        span: Span { start: 10, end: 15 },
    };
    let result = error.span();
    assert_eq!(result.start, 10);
    assert_eq!(result.end, 15);
}

#[test]
fn test_span_boundary_conditions() {
    let error = Error {
        span: Span { start: usize::MAX - 1, end: usize::MAX },
    };
    let result = error.span();
    assert_eq!(result.start, usize::MAX - 1);
    assert_eq!(result.end, usize::MAX);
}

