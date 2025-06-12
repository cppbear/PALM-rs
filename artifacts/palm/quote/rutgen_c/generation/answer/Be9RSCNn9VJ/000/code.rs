// Answer 0

#[test]
fn test_span() {
    use proc_macro2::Span;

    let span = Span::call_site();
    assert_eq!(span.__span(), span);
}

#[test]
fn test_span_clone() {
    use proc_macro2::Span;

    let span = Span::call_site();
    let cloned_span = span.__span();
    assert_eq!(cloned_span, span);
}

