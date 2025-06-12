pub fn shift_remove_index(&mut self, index: usize) -> Option<T> {
        self.map.shift_remove_index(index).map(|(x, ())| x)
    }