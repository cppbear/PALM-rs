pub fn shrink_to(&mut self, min_capacity: usize) {
        self.core.shrink_to(min_capacity);
    }