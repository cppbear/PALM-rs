pub fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsHeaderName,
    {
        key.find(self).is_some()
    }