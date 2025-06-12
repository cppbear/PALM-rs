pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.table
            .try_reserve(additional, make_hasher::<_, V, S>(&self.hash_builder))
    }