pub fn get(&self) -> &T {
        match *self {
            Entry::Occupied(ref entry) => entry.get(),
            Entry::Vacant(ref entry) => entry.get(),
        }
    }