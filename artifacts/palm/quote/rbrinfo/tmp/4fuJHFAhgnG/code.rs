fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i128_suffixed(*self));
    }