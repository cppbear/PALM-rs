pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.0.case_insensitive = yes;
        self
    }