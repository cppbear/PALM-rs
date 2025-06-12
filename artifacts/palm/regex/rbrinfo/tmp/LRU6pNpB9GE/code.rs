pub fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.0.multi_line = yes;
        self
    }