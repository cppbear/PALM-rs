pub fn try_insert<K>(&mut self, key: K, val: T) -> Result<Option<T>, MaxSizeReached>
    where
        K: IntoHeaderName,
    {
        key.try_insert(self, val)
    }