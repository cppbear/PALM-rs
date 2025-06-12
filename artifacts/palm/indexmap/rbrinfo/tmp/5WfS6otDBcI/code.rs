pub fn into_boxed_slice(self) -> Box<Slice<K, V>> {
        Slice::from_boxed(self.into_entries().into_boxed_slice())
    }