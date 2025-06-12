fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens,
    {
        for token in iter {
            token.to_tokens(self);
        }
    }