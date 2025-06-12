pub fn capacity(&self) -> usize {
        usable_capacity(self.indices.len())
    }