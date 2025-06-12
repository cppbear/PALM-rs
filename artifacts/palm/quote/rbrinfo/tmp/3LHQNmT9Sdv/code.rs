fn respan_token_tree(mut token: TokenTree, span: Span) -> TokenTree {
    match &mut token {
        TokenTree::Group(g) => {
            let stream = g
                .stream()
                .into_iter()
                .map(|token| respan_token_tree(token, span))
                .collect();
            *g = Group::new(g.delimiter(), stream);
            g.set_span(span);
        }
        other => other.set_span(span),
    }
    token
}