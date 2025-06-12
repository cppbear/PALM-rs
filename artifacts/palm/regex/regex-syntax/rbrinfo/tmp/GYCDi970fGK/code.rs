pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.swap_greed = if yes { Some(true) } else { None };
        self
    }