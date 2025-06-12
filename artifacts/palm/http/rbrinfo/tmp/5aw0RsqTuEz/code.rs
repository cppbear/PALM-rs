pub fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
        // TODO: This can't overflow if done properly... since the max # of
        // elements is u16::MAX.
        let cap = self
            .entries
            .len()
            .checked_add(additional)
            .ok_or_else(MaxSizeReached::new)?;

        let raw_cap = to_raw_capacity(cap);

        if raw_cap > self.indices.len() {
            let raw_cap = raw_cap
                .checked_next_power_of_two()
                .ok_or_else(MaxSizeReached::new)?;
            if raw_cap > MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            if self.entries.is_empty() {
                self.mask = raw_cap as Size - 1;
                self.indices = vec![Pos::none(); raw_cap].into_boxed_slice();
                self.entries = Vec::with_capacity(usable_capacity(raw_cap));
            } else {
                self.try_grow(raw_cap)?;
            }
        }

        Ok(())
    }