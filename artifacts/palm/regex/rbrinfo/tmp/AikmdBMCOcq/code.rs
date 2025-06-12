pub fn splitn<'r, 't>(
        &'r self,
        text: &'t [u8],
        limit: usize,
    ) -> SplitN<'r, 't> {
        SplitN {
            splits: self.split(text),
            n: limit,
        }
    }