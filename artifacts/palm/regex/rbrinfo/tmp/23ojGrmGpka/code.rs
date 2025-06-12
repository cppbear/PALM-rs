pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder {
        self.0.case_insensitive = yes;
        self
    }