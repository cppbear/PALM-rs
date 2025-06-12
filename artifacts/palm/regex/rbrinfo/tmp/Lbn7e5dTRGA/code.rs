pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder {
        self.0.multi_line = yes;
        self
    }