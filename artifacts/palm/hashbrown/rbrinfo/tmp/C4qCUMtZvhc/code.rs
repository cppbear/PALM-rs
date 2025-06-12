pub fn or_default(self) -> &'a mut V
    where
        K: Hash + From<&'b Q>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(entry) => entry.into_mut(),
            EntryRef::Vacant(entry) => entry.insert(Default::default()),
        }
    }