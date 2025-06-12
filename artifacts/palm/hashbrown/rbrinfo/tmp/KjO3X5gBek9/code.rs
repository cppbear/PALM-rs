pub fn insert_hashed_nocheck(self, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        let &mut (ref mut k, ref mut v) = self.table.insert_entry(
            hash,
            (key, value),
            make_hasher::<_, V, S>(self.hash_builder),
        );
        (k, v)
    }