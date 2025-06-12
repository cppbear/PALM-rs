fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::usize_suffixed(*self));
    }