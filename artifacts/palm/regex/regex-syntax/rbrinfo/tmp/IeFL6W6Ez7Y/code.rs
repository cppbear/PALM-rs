pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.allow_invalid_utf8(yes);
        self
    }