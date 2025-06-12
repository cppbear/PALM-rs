fn init<E>(&self, f: impl FnOnce() -> Result<&'a T, E>) -> Result<&'a T, E> {
        let mut value: &'a T = f()?;
        if let Err(old) = self.compare_exchange(value) {
            value = unsafe { &*old };
        }
        Ok(value)
    }