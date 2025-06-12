// Answer 0

#[test]
fn test_join_spans_empty_tokens() {
    use proc_macro2::TokenStream;
    use proc_macro2::Span;

    let tokens: TokenStream = TokenStream::new();
    let result = join_spans(tokens);
    
    assert_eq!(result, Span::call_site());
}

#[test]
fn test_join_spans_single_span() {
    use proc_macro2::{TokenStream, TokenTree, Span};

    let single_span = TokenTree::Ident(proc_macro2::Ident::new("x", Span::new(1, 2)));
    let tokens: TokenStream = TokenStream::from(single_span);
    
    let result = join_spans(tokens);
    
    assert_eq!(result, Span::new(1, 2));
}

#[test]
fn test_join_spans_multiple_spans() {
    use proc_macro2::{TokenStream, TokenTree, Span};

    let span1 = TokenTree::Ident(proc_macro2::Ident::new("x", Span::new(1, 2)));
    let span2 = TokenTree::Ident(proc_macro2::Ident::new("y", Span::new(3, 4)));
    let tokens: TokenStream = TokenStream::from_iter(vec![span1, span2]);

    let result = join_spans(tokens);
    
    assert!(result != Span::call_site());
}

#[test]
fn test_join_spans_with_same_spans() {
    use proc_macro2::{TokenStream, TokenTree, Span};

    let span = Span::new(1, 2);
    let token1 = TokenTree::Ident(proc_macro2::Ident::new("a", span));
    let token2 = TokenTree::Ident(proc_macro2::Ident::new("b", span));
    let tokens: TokenStream = TokenStream::from_iter(vec![token1, token2]);

    let result = join_spans(tokens);
    
    assert_eq!(result, span);
}

