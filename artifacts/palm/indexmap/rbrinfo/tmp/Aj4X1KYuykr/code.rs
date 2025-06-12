pub fn insert_sorted(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Ord,
    {
        match self.binary_search_keys(&key) {
            Ok(i) => (i, Some(mem::replace(&mut self[i], value))),
            Err(i) => self.insert_before(i, key, value),
        }
    }