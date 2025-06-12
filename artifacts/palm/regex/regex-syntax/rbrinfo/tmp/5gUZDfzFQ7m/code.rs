pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
        self.ast.octal(yes);
        self
    }