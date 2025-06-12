pub fn parse(tokens: &mut TokenStream, s: &str) {
    let s: TokenStream = s.parse().expect("invalid token stream");
    tokens.extend(iter::once(s));
}