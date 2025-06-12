pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.as_entries_mut())
    }