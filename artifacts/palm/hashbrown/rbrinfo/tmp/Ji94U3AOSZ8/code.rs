pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash + From<&'b Q>,
        S: BuildHasher,
    {
        let elem = self.table.table.insert(
            self.hash,
            (self.key.into(), value),
            make_hasher::<_, V, S>(&self.table.hash_builder),
        );
        OccupiedEntry {
            hash: self.hash,
            elem,
            table: self.table,
        }
    }