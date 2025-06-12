pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {
            // Fast path check
            if let Some(value) = self.get() {
                return Ok(value);
            }

            self.0.initialize(f)?;

            // Safe b/c value is initialized.
            debug_assert!(self.0.is_initialized());
            Ok(unsafe { self.get_unchecked() })
        }