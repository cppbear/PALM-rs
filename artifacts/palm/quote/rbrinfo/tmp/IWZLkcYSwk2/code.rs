fn ident_maybe_raw(id: &str, span: Span) -> Ident {
    if let Some(id) = id.strip_prefix("r#") {
        Ident::new_raw(id, span)
    } else {
        Ident::new(id, span)
    }
}