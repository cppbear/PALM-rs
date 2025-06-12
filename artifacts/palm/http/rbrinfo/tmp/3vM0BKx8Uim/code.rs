pub fn uri_ref(&self) -> Option<&Uri> {
        self.inner.as_ref().ok().map(|h| &h.uri)
    }