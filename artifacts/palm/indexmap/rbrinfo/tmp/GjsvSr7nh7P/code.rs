pub fn shift_insert(&mut self, index: usize, value: T) -> bool {
        self.map.shift_insert(index, value, ()).is_none()
    }