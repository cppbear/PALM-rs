fn try_insert_phase_two(
        &mut self,
        key: HeaderName,
        value: T,
        hash: HashValue,
        probe: usize,
        danger: bool,
    ) -> Result<usize, MaxSizeReached> {
        // Push the value and get the index
        let index = self.entries.len();
        self.try_insert_entry(hash, key, value)?;

        let num_displaced = do_insert_phase_two(&mut self.indices, probe, Pos::new(index, hash));

        if danger || num_displaced >= DISPLACEMENT_THRESHOLD {
            // Increase danger level
            self.danger.set_yellow();
        }

        Ok(index)
    }