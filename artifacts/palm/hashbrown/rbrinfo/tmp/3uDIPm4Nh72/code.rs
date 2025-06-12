pub fn insert(self) -> OccupiedEntry<'a, T, S, A>
    where
        T: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(entry) => entry.insert(),
        }
    }