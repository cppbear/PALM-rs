pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.case_insensitive = if yes { Some(true) } else { None };
        self
    }