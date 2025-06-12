pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.map.insert(k, v)
    }