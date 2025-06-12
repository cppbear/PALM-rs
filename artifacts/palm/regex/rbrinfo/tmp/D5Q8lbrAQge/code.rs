pub fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.0.unicode = yes;
        self
    }