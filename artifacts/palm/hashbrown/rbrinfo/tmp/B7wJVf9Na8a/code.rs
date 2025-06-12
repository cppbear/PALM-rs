fn insert_entry(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>
    where
        K: Hash,
        S: BuildHasher,
    {
        let hash = make_hash::<K, S>(self.hash_builder, &key);
        let elem = self.table.insert(
            hash,
            (key, value),
            make_hasher::<_, V, S>(self.hash_builder),
        );
        RawOccupiedEntryMut {
            elem,
            table: self.table,
            hash_builder: self.hash_builder,
        }
    }