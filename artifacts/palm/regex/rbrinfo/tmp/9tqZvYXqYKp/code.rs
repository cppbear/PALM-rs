fn captures_iter(
        self,
        text: &Self::Text,
    ) -> CaptureMatches<Self> {
        CaptureMatches(self.find_iter(text))
    }