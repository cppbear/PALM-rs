pub fn append<K>(&mut self, key: K, value: T) -> bool
    where
        K: IntoHeaderName,
    {
        self.try_append(key, value)
            .expect("size overflows MAX_SIZE")
    }