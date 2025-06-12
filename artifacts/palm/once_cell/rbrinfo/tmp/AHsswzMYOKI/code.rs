fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
        let nz = f()?;
        let mut val = nz.get();
        if let Err(old) = self.compare_exchange(nz) {
            val = old;
        }
        Ok(unsafe { NonZeroUsize::new_unchecked(val) })
    }