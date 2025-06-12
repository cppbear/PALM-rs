pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        let hash = self.hash(&key);
        self.core.entry(hash, key)
    }