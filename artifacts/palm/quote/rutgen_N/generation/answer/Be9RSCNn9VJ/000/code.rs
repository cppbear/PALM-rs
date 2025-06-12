// Answer 0

#[derive(Copy, Clone)]
struct Span;

impl Span {
    fn __span(&self) -> Span {
        *self
    }
}

#[test]
fn test_span_copy() {
    let span = Span;
    let result = span.__span();
    assert_eq!(result, span);
}

#[test]
fn test_span_clone() {
    let span = Span;
    let cloned_span = span.__span();
    assert_eq!(cloned_span, span);
}

