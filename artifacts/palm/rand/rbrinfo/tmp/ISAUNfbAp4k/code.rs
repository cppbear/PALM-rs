pub fn reseed(&mut self) -> Result<(), rand_core::OsError> {
        // SAFETY: We must make sure to stop using `rng` before anyone else
        // creates another mutable reference
        let rng = unsafe { &mut *self.rng.get() };
        rng.reseed()
    }