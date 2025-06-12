fn clone(&self) -> Self {
        Values {
            inner: self.inner.clone(),
        }
    }