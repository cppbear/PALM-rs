// Answer 0

#[test]
fn test_ident_span() {
    use proc_macro2::Ident;
    
    let ident = Ident::new("my_ident", Span::call_site());
    assert_eq!(ident.span(), Some(Span::call_site()));
}

#[test]
fn test_ident_span_empty() {
    use proc_macro2::Ident;

    let ident = Ident::new("", Span::call_site());
    assert_eq!(ident.span(), Some(Span::call_site()));
}

