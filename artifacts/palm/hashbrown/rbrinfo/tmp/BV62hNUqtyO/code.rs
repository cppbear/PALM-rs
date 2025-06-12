fn clone(&self) -> Self {
        Keys {
            inner: self.inner.clone(),
        }
    }