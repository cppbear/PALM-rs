pub fn dot_matches_new_line(
        &mut self,
        yes: bool,
    ) -> &mut ParserBuilder {
        self.hir.dot_matches_new_line(yes);
        self
    }