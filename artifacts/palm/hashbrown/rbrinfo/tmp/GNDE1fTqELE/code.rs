pub fn and_replace_entry_with<F>(self, f: F) -> Self
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        match self {
            Entry::Occupied(entry) => entry.replace_entry_with(f),
            Entry::Vacant(_) => self,
        }
    }