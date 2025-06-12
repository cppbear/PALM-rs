pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.core.try_reserve(additional)
    }