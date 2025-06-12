pub fn insert(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            RawEntryMut::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            RawEntryMut::Vacant(entry) => entry.insert_entry(key, value),
        }
    }