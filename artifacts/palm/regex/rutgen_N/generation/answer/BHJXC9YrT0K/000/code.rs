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
fn test_span_returns_correct_reference() {
    let error = Error {
        span: Span { start: 5, end: 10 },
    };
    let span_ref = error.span();
    assert_eq!(span_ref.start, 5);
    assert_eq!(span_ref.end, 10);
}

#[test]
fn test_span_boundary_conditions() {
    let error_start_zero = Error {
        span: Span { start: 0, end: 0 },
    };
    let span_start_zero_ref = error_start_zero.span();
    assert_eq!(span_start_zero_ref.start, 0);
    assert_eq!(span_start_zero_ref.end, 0);

    let error_end_equal_start = Error {
        span: Span { start: 3, end: 3 },
    };
    let span_end_equal_start_ref = error_end_equal_start.span();
    assert_eq!(span_end_equal_start_ref.start, 3);
    assert_eq!(span_end_equal_start_ref.end, 3);
    
    let error_large_values = Error {
        span: Span { start: usize::MAX - 1, end: usize::MAX },
    };
    let span_large_values_ref = error_large_values.span();
    assert_eq!(span_large_values_ref.start, usize::MAX - 1);
    assert_eq!(span_large_values_ref.end, usize::MAX);
}

