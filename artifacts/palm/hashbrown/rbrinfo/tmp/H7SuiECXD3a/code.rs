pub fn from_key_hashed_nocheck<Q>(self, hash: u64, k: &Q) -> Option<(&'a K, &'a V)>
    where
        Q: Equivalent<K> + ?Sized,
    {
        self.from_hash(hash, equivalent(k))
    }