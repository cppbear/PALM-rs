pub fn insert_hashed_nocheck(self, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V) {
        let hash = HashValue(hash as usize);
        self.map.insert_unique(hash, key, value).into_muts()
    }