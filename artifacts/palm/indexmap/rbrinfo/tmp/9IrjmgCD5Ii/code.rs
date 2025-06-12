pub fn shift_remove_entry(mut self) -> (K, V) {
        self.map.shift_remove_index(self.index).unwrap()
    }