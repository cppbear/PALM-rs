fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u128_suffixed(*self));
    }