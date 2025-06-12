pub fn reserve_exact(&mut self, additional: usize) {
        self.map.reserve_exact(additional);
    }