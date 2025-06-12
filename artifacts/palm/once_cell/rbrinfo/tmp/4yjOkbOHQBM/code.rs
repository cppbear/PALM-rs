pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
            if let Some(old) = self.get() {
                return Err((old, value));
            }

            let slot = unsafe { &mut *self.inner.get() };
            // This is the only place where we set the slot, no races
            // due to reentrancy/concurrency are possible, and we've
            // checked that slot is currently `None`, so this write
            // maintains the `inner`'s invariant.
            *slot = Some(value);
            Ok(unsafe { slot.as_ref().unwrap_unchecked() })
        }