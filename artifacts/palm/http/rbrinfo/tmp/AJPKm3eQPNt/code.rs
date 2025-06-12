pub fn entry<K>(&mut self, key: K) -> Entry<'_, T>
    where
        K: IntoHeaderName,
    {
        key.try_entry(self).expect("size overflows MAX_SIZE")
    }