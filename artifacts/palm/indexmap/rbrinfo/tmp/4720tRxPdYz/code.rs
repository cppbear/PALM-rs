pub fn swap_remove_entry(mut self) -> (K, V) {
        self.map.swap_remove_index(self.index).unwrap()
    }