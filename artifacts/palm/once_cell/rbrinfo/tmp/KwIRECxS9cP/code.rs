pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<Box<T>, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f)
            }
        }