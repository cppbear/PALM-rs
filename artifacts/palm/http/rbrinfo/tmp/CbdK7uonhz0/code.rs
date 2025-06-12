fn try_entry2<K>(&mut self, key: K) -> Result<Entry<'_, T>, MaxSizeReached>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {
        // Ensure that there is space in the map
        self.try_reserve_one()?;

        Ok(insert_phase_one!(
            self,
            key,
            probe,
            pos,
            hash,
            danger,
            Entry::Vacant(VacantEntry {
                map: self,
                hash,
                key: key.into(),
                probe,
                danger,
            }),
            Entry::Occupied(OccupiedEntry {
                map: self,
                index: pos,
                probe,
            }),
            Entry::Vacant(VacantEntry {
                map: self,
                hash,
                key: key.into(),
                probe,
                danger,
            })
        ))
    }