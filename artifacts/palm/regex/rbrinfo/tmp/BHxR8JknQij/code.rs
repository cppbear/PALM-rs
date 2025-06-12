pub fn splitn<'r, 't>(&'r self, text: &'t str, limit: usize)
                         -> SplitN<'r, 't> {
        SplitN {
            splits: self.split(text),
            n: limit,
        }
    }