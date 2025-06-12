pub fn from_key_hashed_nocheck<Q>(self, hash: u64, key: &Q) -> RawEntryMut<'a, K, V, S>
    where
        Q: ?Sized + Equivalent<K>,
    {
        self.from_hash(hash, |k| Q::equivalent(key, k))
    }