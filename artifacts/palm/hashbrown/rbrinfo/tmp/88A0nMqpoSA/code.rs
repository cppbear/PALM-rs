pub fn or_insert(self, default: T) -> OccupiedEntry<'a, T, A> {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(entry) => entry.insert(default),
        }
    }