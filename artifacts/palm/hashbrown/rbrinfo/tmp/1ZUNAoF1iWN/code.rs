pub fn shrink_to(&mut self, min_capacity: usize, hasher: impl Fn(&T) -> u64) {
        self.raw.shrink_to(min_capacity, hasher);
    }