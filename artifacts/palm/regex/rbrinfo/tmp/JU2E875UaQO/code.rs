pub fn replace_all<'t, R: Replacer>(
        &self,
        text: &'t str,
        rep: R,
    ) -> Cow<'t, str> {
        self.replacen(text, 0, rep)
    }