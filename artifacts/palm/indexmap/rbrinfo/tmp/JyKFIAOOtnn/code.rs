pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
        let Self { map, hash, key } = self;
        map.insert_unique(hash, key, value)
    }