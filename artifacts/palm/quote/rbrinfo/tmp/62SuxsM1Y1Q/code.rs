fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u8_suffixed(*self));
    }