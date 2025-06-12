pub fn into_keys(self) -> IntoKeys<K, V, A> {
        IntoKeys {
            inner: self.into_iter(),
        }
    }