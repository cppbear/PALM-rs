fn try_insert2<K>(&mut self, key: K, value: T) -> Result<Option<T>, MaxSizeReached>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {
        self.try_reserve_one()?;

        Ok(insert_phase_one!(
            self,
            key,
            probe,
            pos,
            hash,
            danger,
            // Vacant
            {
                let _ = danger; // Make lint happy
                let index = self.entries.len();
                self.try_insert_entry(hash, key.into(), value)?;
                self.indices[probe] = Pos::new(index, hash);
                None
            },
            // Occupied
            Some(self.insert_occupied(pos, value)),
            // Robinhood
            {
                self.try_insert_phase_two(key.into(), value, hash, probe, danger)?;
                None
            }
        ))
    }