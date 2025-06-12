pub fn shift_insert_hashed_nocheck(
        mut self,
        index: usize,
        hash: u64,
        key: K,
        value: V,
    ) -> (&'a mut K, &'a mut V) {
        let hash = HashValue(hash as usize);
        self.map.shift_insert_unique(index, hash, key, value);
        self.map.entries[index].muts()
    }