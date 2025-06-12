pub fn size_limit(&mut self, limit: usize) -> &mut RegexSetBuilder {
        self.0.size_limit = limit;
        self
    }