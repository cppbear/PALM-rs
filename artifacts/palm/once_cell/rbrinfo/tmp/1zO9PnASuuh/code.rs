pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            // Note that *some* forms of reentrant initialization might lead to
            // UB (see `reentrant_init` test). I believe that just removing this
            // `assert`, while keeping `set/get` would be sound, but it seems
            // better to panic, rather than to silently use an old value.
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(unsafe { self.get().unwrap_unchecked() })
        }