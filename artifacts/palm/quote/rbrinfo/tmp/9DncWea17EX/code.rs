pub fn push_underscore_spanned(tokens: &mut TokenStream, span: Span) {
    tokens.append(Ident::new("_", span));
}