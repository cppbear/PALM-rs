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
    /// Return the span at which this error occurred.
    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[test]
fn test_span_valid() {
    let error = Error {
        span: Span { start: 0, end: 5 },
    };
    let span = error.span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 5);
}

#[test]
fn test_span_zero_start() {
    let error = Error {
        span: Span { start: 0, end: 10 },
    };
    let span = error.span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 10);
}

#[test]
fn test_span_equal_start_end() {
    let error = Error {
        span: Span { start: 5, end: 5 },
    };
    let span = error.span();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 5);
}

#[test]
fn test_span_large_values() {
    let error = Error {
        span: Span { start: usize::MAX - 1, end: usize::MAX },
    };
    let span = error.span();
    assert_eq!(span.start, usize::MAX - 1);
    assert_eq!(span.end, usize::MAX);
}

