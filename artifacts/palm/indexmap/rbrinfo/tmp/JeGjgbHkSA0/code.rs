pub fn into_keys(self: Box<Self>) -> IntoKeys<K, V> {
        IntoKeys::new(self.into_entries())
    }