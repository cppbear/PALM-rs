pub fn or_insert_with(self, default: impl FnOnce() -> T) -> OccupiedEntry<'a, T, A> {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }