pub fn try_append<K>(&mut self, key: K, value: T) -> Result<bool, MaxSizeReached>
    where
        K: IntoHeaderName,
    {
        key.try_append(self, value)
    }