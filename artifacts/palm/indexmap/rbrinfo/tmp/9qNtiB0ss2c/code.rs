pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {
        self.get_index_entry(self.len().checked_sub(1)?)
    }