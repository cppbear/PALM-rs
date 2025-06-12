pub fn unicode(&mut self, yes: bool) -> &mut RegexBuilder {
        self.0.unicode = yes;
        self
    }