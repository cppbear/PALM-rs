pub fn or_insert_with<F>(self, call: F) -> (&'a mut K, &'a mut V)
    where
        F: FnOnce() -> (K, V),
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Self::Occupied(entry) => entry.into_key_value_mut(),
            Self::Vacant(entry) => {
                let (key, value) = call();
                entry.insert(key, value)
            }
        }
    }