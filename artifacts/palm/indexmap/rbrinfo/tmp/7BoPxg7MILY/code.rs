pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {
        let hash = self.hash(&key);
        self.core.insert_full(hash, key, value)
    }