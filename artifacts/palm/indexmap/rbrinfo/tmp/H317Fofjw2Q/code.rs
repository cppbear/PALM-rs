pub fn swap_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::new(entry.into_table(), self.entries).swap_remove_finish(index)
    }