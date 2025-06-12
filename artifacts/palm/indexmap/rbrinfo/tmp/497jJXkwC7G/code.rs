pub fn or_insert(self, default_key: K, default_value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Self::Occupied(entry) => entry.into_key_value_mut(),
            Self::Vacant(entry) => entry.insert(default_key, default_value),
        }
    }