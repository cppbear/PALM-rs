pub fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.as_entries())
    }