pub fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.swap_greed(yes);
        self
    }