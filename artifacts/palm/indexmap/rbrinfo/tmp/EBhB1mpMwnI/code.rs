pub fn reserve_exact(&mut self, additional: usize) {
        self.core.reserve_exact(additional);
    }