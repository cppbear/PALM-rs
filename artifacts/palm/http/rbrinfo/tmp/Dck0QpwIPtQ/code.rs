pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0.get())
    }