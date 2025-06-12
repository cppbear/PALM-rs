pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.multi_line = if yes { Some(true) } else { None };
        self
    }