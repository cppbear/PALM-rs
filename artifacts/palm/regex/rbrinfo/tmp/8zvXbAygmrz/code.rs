pub fn size_limit(&mut self, limit: usize) -> &mut RegexBuilder {
        self.0.size_limit = limit;
        self
    }