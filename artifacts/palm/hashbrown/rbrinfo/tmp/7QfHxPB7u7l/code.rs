pub fn key(&self) -> &Q
    where
        K: Borrow<Q>,
    {
        match *self {
            EntryRef::Occupied(ref entry) => entry.key().borrow(),
            EntryRef::Vacant(ref entry) => entry.key(),
        }
    }