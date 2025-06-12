fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u32_suffixed(*self));
    }