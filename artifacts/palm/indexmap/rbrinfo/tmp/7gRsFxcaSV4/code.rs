pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {
        self.get_index_entry(0)
    }