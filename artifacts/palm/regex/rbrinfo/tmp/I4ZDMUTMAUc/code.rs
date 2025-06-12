pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder {
        self.0.dot_matches_new_line = yes;
        self
    }