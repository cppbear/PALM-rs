pub fn dot_matches_new_line(
        &mut self,
        yes: bool,
    ) -> &mut TranslatorBuilder {
        self.flags.dot_matches_new_line = if yes { Some(true) } else { None };
        self
    }