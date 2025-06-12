pub fn insert(self, value: T) -> OccupiedEntry<'a, T, A> {
        match self {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() = value;
                entry
            }
            Entry::Vacant(entry) => entry.insert(value),
        }
    }