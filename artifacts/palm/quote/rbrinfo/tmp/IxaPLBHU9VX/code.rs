pub fn push_group_spanned(
    tokens: &mut TokenStream,
    span: Span,
    delimiter: Delimiter,
    inner: TokenStream,
) {
    let mut g = Group::new(delimiter, inner);
    g.set_span(span);
    tokens.append(g);
}