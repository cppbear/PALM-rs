// Answer 0

#[test]
fn test_join_spans_empty() {
    use quote::TokenStream;
    use proc_macro2::{Span, TokenTree};

    let tokens: TokenStream = TokenStream::new(); // Create an empty TokenStream
    let result = join_spans(tokens);
    assert_eq!(result, Span::call_site());
}

#[test]
fn test_join_spans_single() {
    use quote::TokenStream;
    use proc_macro2::{Span, TokenTree};

    let tokens: TokenStream = TokenStream::from(TokenTree::Ident(Ident::new("a", Span::from_u34(0)))); // Create a TokenStream with a single identifier
    let result = join_spans(tokens);
    assert_eq!(result, Span::from_u34(0)); // Check that the span matches the identifier's span
}

#[test]
fn test_join_spans_multiple() {
    use quote::TokenStream;
    use proc_macro2::{Span, TokenTree};

    let tokens: TokenStream = TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("a", Span::from_u34(0))),
        TokenTree::Ident(Ident::new("b", Span::from_u34(10))),
    ]);
    
    let result = join_spans(tokens);
    assert_eq!(result, Span::from(0..10)); // Check that the span is combined correctly
}

#[test]
fn test_join_spans_with_none() {
    use quote::TokenStream;
    use proc_macro2::{Span, TokenTree};

    let tokens: TokenStream = TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("a", Span::from_u34(0))),
        TokenTree::Ident(Ident::new("b", Span::call_site())),
    ]);
    
    let result = join_spans(tokens);
    assert_eq!(result, Span::from_u34(0)); // Should return the span of 'a'
}

