pub fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.unicode = if yes { None } else { Some(false) };
        self
    }