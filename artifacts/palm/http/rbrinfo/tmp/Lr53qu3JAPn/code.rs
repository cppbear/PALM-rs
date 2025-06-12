pub fn version_ref(&self) -> Option<&Version> {
        self.inner.as_ref().ok().map(|h| &h.version)
    }