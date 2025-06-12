pub fn allow_invalid_utf8(
        &mut self,
        yes: bool,
    ) -> &mut TranslatorBuilder {
        self.allow_invalid_utf8 = yes;
        self
    }