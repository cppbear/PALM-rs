pub fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.iter.as_slice())
    }