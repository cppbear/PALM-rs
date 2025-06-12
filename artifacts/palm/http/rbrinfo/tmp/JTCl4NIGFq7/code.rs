pub fn insert_mult(&mut self, value: T) -> ValueDrain<'_, T> {
        self.map.insert_occupied_mult(self.index, value)
    }