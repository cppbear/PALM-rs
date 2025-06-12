pub fn insert<K>(&mut self, key: K, val: T) -> Option<T>
    where
        K: IntoHeaderName,
    {
        self.try_insert(key, val).expect("size overflows MAX_SIZE")
    }