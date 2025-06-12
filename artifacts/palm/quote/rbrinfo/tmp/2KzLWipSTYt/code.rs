fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_str().to_tokens(tokens);
    }