pub fn insert_full(&mut self, value: T) -> (usize, bool) {
        let (index, existing) = self.map.insert_full(value, ());
        (index, existing.is_none())
    }