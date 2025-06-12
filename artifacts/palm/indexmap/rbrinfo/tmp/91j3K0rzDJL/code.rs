pub fn shift_insert(self, index: usize, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        let mut h = self.hash_builder.build_hasher();
        key.hash(&mut h);
        self.shift_insert_hashed_nocheck(index, h.finish(), key, value)
    }