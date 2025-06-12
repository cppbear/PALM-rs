pub fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.0.octal = yes;
        self
    }