// Answer 0

#[test]
fn test_span_identity() {
    use proc_macro2::Span;
    
    let span = Span::call_site();
    let returned_span = span.__span();
    assert_eq!(returned_span, span);
}

#[test]
fn test_span_from_char() {
    use proc_macro2::Span;
    
    let span = Span::from_char('a');
    let returned_span = span.__span();
    assert_eq!(returned_span, span);
}

#[test]
fn test_span_invalid() {
    use proc_macro2::Span;
    
    let span = Span::def_site();
    let returned_span = span.__span();
    assert_eq!(returned_span, span);
}

