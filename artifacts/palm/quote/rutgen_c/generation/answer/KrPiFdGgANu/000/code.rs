// Answer 0

#[test]
fn test_ident_span() {
    use proc_macro2::Ident;

    let ident = Ident::new("example_identifier", Span::call_site());
    let span = ident.span();
    
    assert!(span.is_some());
}

#[test]
fn test_ident_fragment_span() {
    use proc_macro2::Ident;

    let ident = Ident::new("another_example", Span::call_site());
    let ident_fragment: &dyn IdentFragment = &ident;
    
    assert!(ident_fragment.span().is_some());
}

