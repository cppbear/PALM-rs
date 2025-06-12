pub fn mk_ident(id: &str, span: Option<Span>) -> Ident {
    let span = span.unwrap_or_else(Span::call_site);
    ident_maybe_raw(id, span)
}