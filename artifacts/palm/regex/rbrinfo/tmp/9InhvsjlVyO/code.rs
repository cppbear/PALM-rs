pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder {
        self.0.ignore_whitespace = yes;
        self
    }