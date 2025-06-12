pub fn reserve(&mut self, additional: usize, hasher: impl Fn(&T) -> u64) {
        if unlikely(additional > self.table.growth_left) {
            // Avoid `Result::unwrap_or_else` because it bloats LLVM IR.
            unsafe {
                // SAFETY: The [`RawTableInner`] must already have properly initialized control
                // bytes since we will never expose RawTable::new_uninitialized in a public API.
                if self
                    .reserve_rehash(additional, hasher, Fallibility::Infallible)
                    .is_err()
                {
                    // SAFETY: All allocation errors will be caught inside `RawTableInner::reserve_rehash`.
                    hint::unreachable_unchecked()
                }
            }
        }
    }