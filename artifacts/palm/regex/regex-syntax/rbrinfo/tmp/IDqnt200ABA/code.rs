pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.case_insensitive(yes);
        self
    }