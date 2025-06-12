pub fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.unicode(yes);
        self
    }