fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i16_suffixed(*self));
    }