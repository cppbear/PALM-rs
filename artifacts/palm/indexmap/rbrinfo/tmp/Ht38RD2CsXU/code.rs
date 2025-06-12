pub fn from_hash<F>(self, hash: u64, mut is_match: F) -> RawEntryMut<'a, K, V, S>
    where
        F: FnMut(&K) -> bool,
    {
        let ref_entries = &*self.map.core.entries;
        let eq = move |&i: &usize| is_match(&ref_entries[i].key);
        match self.map.core.indices.find_entry(hash, eq) {
            Ok(index) => RawEntryMut::Occupied(RawOccupiedEntryMut {
                entries: &mut self.map.core.entries,
                index,
                hash_builder: PhantomData,
            }),
            Err(absent) => RawEntryMut::Vacant(RawVacantEntryMut {
                map: RefMut::new(absent.into_table(), &mut self.map.core.entries),
                hash_builder: &self.map.hash_builder,
            }),
        }
    }