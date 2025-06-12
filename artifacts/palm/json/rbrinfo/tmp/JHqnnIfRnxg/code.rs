pub fn insert(self, value: Value) -> &'a mut Value {
        self.vacant.insert(value)
    }