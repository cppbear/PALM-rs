fn join_spans(tokens: TokenStream) -> Span {
    let mut iter = tokens.into_iter().map(|tt| tt.span());

    let first = match iter.next() {
        Some(span) => span,
        None => return Span::call_site(),
    };

    iter.fold(None, |_prev, next| Some(next))
        .and_then(|last| first.join(last))
        .unwrap_or(first)
}