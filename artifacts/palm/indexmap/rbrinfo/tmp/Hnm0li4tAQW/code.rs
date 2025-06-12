pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(&self.entries)
    }