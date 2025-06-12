pub fn dfa_size_limit(&mut self, limit: usize) -> &mut RegexSetBuilder {
        self.0.dfa_size_limit = limit;
        self
    }