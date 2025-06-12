pub fn shift_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::new(entry.into_table(), self.entries).shift_remove_finish(index)
    }