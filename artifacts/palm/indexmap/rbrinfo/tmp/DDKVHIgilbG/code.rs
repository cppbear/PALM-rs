pub fn swap_remove_index(&mut self, index: usize) -> Option<T> {
        self.map.swap_remove_index(index).map(|(x, ())| x)
    }