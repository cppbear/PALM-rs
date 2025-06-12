pub fn push_lifetime_spanned(tokens: &mut TokenStream, span: Span, lifetime: &str) {
    tokens.extend([
        TokenTree::Punct({
            let mut apostrophe = Punct::new('\'', Spacing::Joint);
            apostrophe.set_span(span);
            apostrophe
        }),
        TokenTree::Ident(Ident::new(&lifetime[1..], span)),
    ]);
}