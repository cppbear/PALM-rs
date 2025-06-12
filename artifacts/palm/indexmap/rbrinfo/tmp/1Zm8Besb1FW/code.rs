pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {
        let slice = crate::map::Slice::from_slice(self.map.entries);
        let i = slice.binary_search_keys(&self.key).unwrap_err();
        (i, self.shift_insert(i, value))
    }