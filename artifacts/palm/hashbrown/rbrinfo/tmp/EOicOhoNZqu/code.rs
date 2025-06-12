pub fn capacity(&self) -> usize {
        self.table.items + self.table.growth_left
    }