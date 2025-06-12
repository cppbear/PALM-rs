fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get(index) {
            Some(entry) => {
                erase_index(self.indices, entry.hash, index);
                Some(self.shift_remove_finish(index))
            }
            None => None,
        }
    }