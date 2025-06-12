pub fn replace<'t, R: Replacer>(
        &self,
        text: &'t str,
        rep: R,
    ) -> Cow<'t, str> {
        self.replacen(text, 1, rep)
    }