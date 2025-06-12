pub fn insert(&mut self, value: Value) -> Value {
        self.occupied.insert(value)
    }