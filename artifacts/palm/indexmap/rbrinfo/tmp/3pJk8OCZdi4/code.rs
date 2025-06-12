pub fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.core.shift_remove_index(index)
    }