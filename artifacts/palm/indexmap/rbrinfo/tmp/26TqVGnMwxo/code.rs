fn into_ref_mut(self) -> RefMut<'a, K, V> {
        RefMut::new(self.index.into_table(), self.entries)
    }