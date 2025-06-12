pub(crate) fn entry(&mut self, hash: HashValue, key: K) -> Entry<'_, K, V>
    where
        K: Eq,
    {
        let entries = &mut self.entries;
        let eq = equivalent(&key, entries);
        match self.indices.find_entry(hash.get(), eq) {
            Ok(index) => Entry::Occupied(OccupiedEntry { entries, index }),
            Err(absent) => Entry::Vacant(VacantEntry {
                map: RefMut::new(absent.into_table(), entries),
                hash,
                key,
            }),
        }
    }