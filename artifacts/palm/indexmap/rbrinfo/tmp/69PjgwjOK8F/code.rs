pub fn into_values(self: Box<Self>) -> IntoValues<K, V> {
        IntoValues::new(self.into_entries())
    }