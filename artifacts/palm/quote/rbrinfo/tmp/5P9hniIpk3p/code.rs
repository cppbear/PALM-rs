fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i8_suffixed(*self));
    }