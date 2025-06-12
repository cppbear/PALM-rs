fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
        // Use a soft-limit on the maximum capacity, but if the caller explicitly
        // requested more, do it and let them have the resulting error.
        let new_capacity = Ord::min(self.indices.capacity(), Self::MAX_ENTRIES_CAPACITY);
        let try_add = new_capacity - self.entries.len();
        if try_add > additional && self.entries.try_reserve_exact(try_add).is_ok() {
            return Ok(());
        }
        self.entries
            .try_reserve_exact(additional)
            .map_err(TryReserveError::from_alloc)
    }