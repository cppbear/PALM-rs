pub fn push_lifetime(tokens: &mut TokenStream, lifetime: &str) {
    tokens.extend([
        TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
        TokenTree::Ident(Ident::new(&lifetime[1..], Span::call_site())),
    ]);
}