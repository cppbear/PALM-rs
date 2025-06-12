pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
        self.ignore_whitespace = yes;
        self
    }