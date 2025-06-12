fn try_append2<K>(&mut self, key: K, value: T) -> Result<bool, MaxSizeReached>
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
                let _ = danger;
                let index = self.entries.len();
                self.try_insert_entry(hash, key.into(), value)?;
                self.indices[probe] = Pos::new(index, hash);
                false
            },
            // Occupied
            {
                append_value(pos, &mut self.entries[pos], &mut self.extra_values, value);
                true
            },
            // Robinhood
            {
                self.try_insert_phase_two(key.into(), value, hash, probe, danger)?;

                false
            }
        ))
    }