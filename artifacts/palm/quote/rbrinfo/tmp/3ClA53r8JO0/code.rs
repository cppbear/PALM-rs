fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u64_suffixed(*self));
    }