pub fn entry<S>(&mut self, key: S) -> Entry
    where
        S: Into<String>,
    {
        #[cfg(not(feature = "preserve_order"))]
        use alloc::collections::btree_map::Entry as EntryImpl;
        #[cfg(feature = "preserve_order")]
        use indexmap::map::Entry as EntryImpl;

        match self.map.entry(key.into()) {
            EntryImpl::Vacant(vacant) => Entry::Vacant(VacantEntry { vacant }),
            EntryImpl::Occupied(occupied) => Entry::Occupied(OccupiedEntry { occupied }),
        }
    }