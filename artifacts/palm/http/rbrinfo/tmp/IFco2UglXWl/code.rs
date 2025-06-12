pub fn insert(&mut self, value: T) -> T {
        self.map.insert_occupied(self.index, value)
    }