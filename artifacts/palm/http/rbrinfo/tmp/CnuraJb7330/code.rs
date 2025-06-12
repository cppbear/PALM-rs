pub fn scheme(&self) -> Option<&Scheme> {
        if self.scheme.inner.is_none() {
            None
        } else {
            Some(&self.scheme)
        }
    }