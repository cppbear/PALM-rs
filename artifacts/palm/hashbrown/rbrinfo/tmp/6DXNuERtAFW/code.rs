pub fn try_reserve(
        &mut self,
        additional: usize,
        hasher: impl Fn(&T) -> u64,
    ) -> Result<(), TryReserveError> {
        if additional > self.table.growth_left {
            // SAFETY: The [`RawTableInner`] must already have properly initialized control
            // bytes since we will never expose RawTable::new_uninitialized in a public API.
            unsafe { self.reserve_rehash(additional, hasher, Fallibility::Fallible) }
        } else {
            Ok(())
        }
    }