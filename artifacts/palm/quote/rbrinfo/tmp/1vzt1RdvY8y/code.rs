pub fn parse_spanned(tokens: &mut TokenStream, span: Span, s: &str) {
    let s: TokenStream = s.parse().expect("invalid token stream");
    tokens.extend(s.into_iter().map(|t| respan_token_tree(t, span)));
}