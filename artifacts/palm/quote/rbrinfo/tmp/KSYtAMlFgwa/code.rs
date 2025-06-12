pub fn push_ident_spanned(tokens: &mut TokenStream, span: Span, s: &str) {
    tokens.append(ident_maybe_raw(s, span));
}