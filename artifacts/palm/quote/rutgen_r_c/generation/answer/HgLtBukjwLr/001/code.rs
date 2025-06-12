// Answer 0

#[test]
fn test_join_spans_empty_tokens() {
    use proc_macro2::{TokenStream, Span};
    let tokens: TokenStream = TokenStream::new();
    let result = join_spans(tokens);
    assert_eq!(result, Span::call_site());
}

#[test]
fn test_join_spans_single_token() {
    use proc_macro2::{TokenStream, Span, TokenTree};
    let tokens: TokenStream = TokenStream::from(TokenTree::Ident(proc_macro2::Ident::new("a", Span::from((1, 1)))));
    let result = join_spans(tokens);
    assert_eq!(result, Span::from((1, 1)));
}

#[test]
fn test_join_spans_multiple_tokens() {
    use proc_macro2::{TokenStream, Span, TokenTree};
    let token_a = TokenTree::Ident(proc_macro2::Ident::new("a", Span::from((1, 1))));
    let token_b = TokenTree::Ident(proc_macro2::Ident::new("b", Span::from((2, 2))));
    let tokens: TokenStream = TokenStream::from_iter(vec![token_a, token_b]);
    let result = join_spans(tokens);
    assert_eq!(result, Span::join(Span::from((1, 1)), Span::from((2, 2))).unwrap());
}

#[test]
fn test_join_spans_overlapping_tokens() {
    use proc_macro2::{TokenStream, Span, TokenTree};
    let token_a = TokenTree::Ident(proc_macro2::Ident::new("a", Span::from((1, 1))));
    let token_b = TokenTree::Ident(proc_macro2::Ident::new("b", Span::from((1, 2))));
    let tokens: TokenStream = TokenStream::from_iter(vec![token_a, token_b]);
    let result = join_spans(tokens);
    assert_eq!(result, Span::from((1, 2)));
}

