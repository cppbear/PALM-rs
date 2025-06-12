pub fn captures_iter<'r, 't>(
        &'r self,
        text: &'t [u8],
    ) -> CaptureMatches<'r, 't> {
        CaptureMatches(self.0.searcher().captures_iter(text))
    }