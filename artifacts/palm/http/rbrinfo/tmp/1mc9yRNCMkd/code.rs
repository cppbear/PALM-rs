pub fn try_insert(self, value: T) -> Result<&'a mut T, MaxSizeReached> {
        // Ensure that there is space in the map
        let index =
            self.map
                .try_insert_phase_two(self.key, value, self.hash, self.probe, self.danger)?;

        Ok(&mut self.map.entries[index].value)
    }