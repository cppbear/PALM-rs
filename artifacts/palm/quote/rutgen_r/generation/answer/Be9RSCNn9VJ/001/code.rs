// Answer 0

#[derive(Clone, Copy)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

impl From<Span> for Span {
    fn from(span: Span) -> Self {
        span
    }
}

fn __span(span: &Span) -> Span {
    *span
}

#[test]
fn test_span_with_valid_bounds() {
    let span = Span::new(0, 10);
    let result = __span(&span);
    assert_eq!(result, span);
}

#[test]
fn test_span_with_zero_bounds() {
    let span = Span::new(0, 0);
    let result = __span(&span);
    assert_eq!(result, span);
}

#[test]
fn test_span_with_negative_bounds() {
    let span = Span::new(usize::MAX, usize::MAX);
    let result = __span(&span);
    assert_eq!(result, span);
}

