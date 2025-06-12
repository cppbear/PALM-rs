pub fn into_slice(self) -> &'a mut Slice<K, V> {
        Slice::from_mut_slice(self.iter.into_slice())
    }