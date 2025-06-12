pub fn insert_sorted(&mut self, value: T) -> (usize, bool)
    where
        T: Ord,
    {
        let (index, existing) = self.map.insert_sorted(value, ());
        (index, existing.is_none())
    }