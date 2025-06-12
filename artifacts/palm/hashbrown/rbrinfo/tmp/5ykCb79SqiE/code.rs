pub fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        let hash = make_hash::<K, S>(self.hash_builder, &key);
        self.insert_hashed_nocheck(hash, key, value)
    }