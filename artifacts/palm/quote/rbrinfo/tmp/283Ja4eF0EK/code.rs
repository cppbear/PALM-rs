fn to_tokens(&self, tokens: &mut TokenStream) {
        let word = if *self { "true" } else { "false" };
        tokens.append(Ident::new(word, Span::call_site()));
    }