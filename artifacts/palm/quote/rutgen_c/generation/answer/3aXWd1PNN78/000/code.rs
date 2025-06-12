// Answer 0

#[test]
fn test_into_token_stream() {
    let token_stream: TokenStream = TokenStream::new();
    let result: TokenStream = token_stream.clone().into_token_stream();
    assert_eq!(result.to_string(), token_stream.to_string());
}

#[test]
fn test_into_token_stream_non_empty() {
    let token_stream: TokenStream = TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("foo", Span::call_site())),
        TokenTree::Punct(Punct::new(';', proc_macro2::Spacing::Alone)),
    ]);
    let result: TokenStream = token_stream.clone().into_token_stream();
    assert_eq!(result.to_string(), token_stream.to_string());
}

