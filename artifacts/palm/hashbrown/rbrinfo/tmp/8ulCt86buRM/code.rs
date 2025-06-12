pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }