pub fn or_insert(self, default_key: K, default_val: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            RawEntryMut::Occupied(entry) => entry.into_key_value(),
            RawEntryMut::Vacant(entry) => entry.insert(default_key, default_val),
        }
    }