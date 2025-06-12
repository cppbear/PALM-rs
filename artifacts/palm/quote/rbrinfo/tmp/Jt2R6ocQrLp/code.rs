fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u16_suffixed(*self));
    }