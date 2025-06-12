pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.0.ignore_whitespace = yes;
        self
    }