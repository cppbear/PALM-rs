pub fn captures_iter<'r, 't>(
        &'r self,
        text: &'t str,
    ) -> CaptureMatches<'r, 't> {
        CaptureMatches(self.0.searcher_str().captures_iter(text))
    }