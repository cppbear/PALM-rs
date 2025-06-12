fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(iter::once(self.clone()));
    }