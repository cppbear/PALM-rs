pub fn insert(self) -> OccupiedEntry<'a, T, S, A>
    where
        T: Hash,
        S: BuildHasher,
    {
        OccupiedEntry {
            inner: self.inner.insert_entry(()),
        }
    }