pub fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder {
        self.0.nest_limit = limit;
        self
    }