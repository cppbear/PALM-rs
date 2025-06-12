pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        Slice::from_mut_slice(self.as_entries_mut())
    }