pub fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        let mut h = self.hash_builder.build_hasher();
        key.hash(&mut h);
        self.insert_hashed_nocheck(h.finish(), key, value)
    }