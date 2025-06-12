fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(t) = self {
            t.to_tokens(tokens);
        }
    }