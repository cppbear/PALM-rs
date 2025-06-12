fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::f64_suffixed(*self));
    }