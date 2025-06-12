pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.core.swap_remove_index(index)
    }