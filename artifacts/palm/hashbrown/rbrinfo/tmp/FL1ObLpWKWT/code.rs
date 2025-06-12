pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash + From<&'b Q>,
        S: BuildHasher,
    {
        let table = &mut self.table.table;
        let entry = table.insert_entry(
            self.hash,
            (self.key.into(), value),
            make_hasher::<_, V, S>(&self.table.hash_builder),
        );
        &mut entry.1
    }