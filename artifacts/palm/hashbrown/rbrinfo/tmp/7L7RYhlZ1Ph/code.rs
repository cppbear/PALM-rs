pub fn insert(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash + From<&'b Q>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            EntryRef::Vacant(entry) => entry.insert_entry(value),
        }
    }