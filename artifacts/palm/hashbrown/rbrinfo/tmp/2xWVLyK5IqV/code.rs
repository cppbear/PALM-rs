pub fn or_insert_with_key<F: FnOnce(&Q) -> V>(self, default: F) -> &'a mut V
    where
        K: Hash + Borrow<Q> + From<&'b Q>,
        S: BuildHasher,
    {
        match self {
            EntryRef::Occupied(entry) => entry.into_mut(),
            EntryRef::Vacant(entry) => {
                let value = default(entry.key);
                entry.insert(value)
            }
        }
    }