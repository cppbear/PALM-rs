pub fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.0.swap_greed = yes;
        self
    }