pub fn from_key<Q>(self, k: &Q) -> Option<(&'a K, &'a V)>
    where
        S: BuildHasher,
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hash = make_hash::<Q, S>(&self.map.hash_builder, k);
        self.from_key_hashed_nocheck(hash, k)
    }