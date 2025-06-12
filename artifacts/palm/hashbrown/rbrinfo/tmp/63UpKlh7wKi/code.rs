pub fn from_key<Q>(self, k: &Q) -> RawEntryMut<'a, K, V, S, A>
    where
        S: BuildHasher,
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hash = make_hash::<Q, S>(&self.map.hash_builder, k);
        self.from_key_hashed_nocheck(hash, k)
    }