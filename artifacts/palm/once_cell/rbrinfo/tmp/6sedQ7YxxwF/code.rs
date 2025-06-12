pub fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>
    where
        F: FnOnce() -> Result<NonZeroUsize, E>,
    {
        match self.get() {
            Some(it) => Ok(it),
            None => self.init(f),
        }
    }