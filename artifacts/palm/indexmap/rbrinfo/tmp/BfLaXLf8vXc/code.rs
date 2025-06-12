pub fn index(&self) -> usize {
        match self {
            Self::Occupied(entry) => entry.index(),
            Self::Vacant(entry) => entry.index(),
        }
    }