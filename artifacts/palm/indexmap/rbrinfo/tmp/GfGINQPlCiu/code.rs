pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {
        if index >= self.len() {
            return None;
        }
        Some(IndexedEntry::new(&mut self.core, index))
    }