pub fn from_key_hashed_nocheck<Q>(self, hash: u64, k: &Q) -> RawEntryMut<'a, K, V, S, A>
    where
        Q: Equivalent<K> + ?Sized,
    {
        self.from_hash(hash, equivalent(k))
    }