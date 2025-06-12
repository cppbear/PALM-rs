pub fn insert_before(&mut self, index: usize, value: T) -> (usize, bool) {
        let (index, existing) = self.map.insert_before(index, value, ());
        (index, existing.is_none())
    }