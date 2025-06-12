fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i64_suffixed(*self));
    }