pub fn replace_all<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        rep: R,
    ) -> Cow<'t, [u8]> {
        self.replacen(text, 0, rep)
    }