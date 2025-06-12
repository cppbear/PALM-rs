pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            EntryRef::Occupied(mut entry) => {
                f(entry.get_mut());
                EntryRef::Occupied(entry)
            }
            EntryRef::Vacant(entry) => EntryRef::Vacant(entry),
        }
    }