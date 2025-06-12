pub fn get<K>(&self, key: K) -> Option<&T>
    where
        K: AsHeaderName,
    {
        self.get2(&key)
    }