pub fn octal(&mut self, yes: bool) -> &mut RegexBuilder {
        self.0.octal = yes;
        self
    }