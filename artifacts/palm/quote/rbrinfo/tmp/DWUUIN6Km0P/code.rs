pub fn push_underscore(tokens: &mut TokenStream) {
    push_underscore_spanned(tokens, Span::call_site());
}