pub fn try_insert_entry(self, value: T) -> Result<OccupiedEntry<'a, T>, MaxSizeReached> {
        // Ensure that there is space in the map
        let index =
            self.map
                .try_insert_phase_two(self.key, value, self.hash, self.probe, self.danger)?;

        Ok(OccupiedEntry {
            map: self.map,
            index,
            probe: self.probe,
        })
    }