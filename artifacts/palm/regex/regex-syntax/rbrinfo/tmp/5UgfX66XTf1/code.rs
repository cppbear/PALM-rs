pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
        self.ast.ignore_whitespace(yes);
        self
    }