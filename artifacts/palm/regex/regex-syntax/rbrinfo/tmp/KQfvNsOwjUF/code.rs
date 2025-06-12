pub fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.multi_line(yes);
        self
    }