unsafe fn drop_elements<T>(&mut self) {
        // Check that `self.items != 0`. Protects against the possibility
        // of creating an iterator on an table with uninitialized control bytes.
        if T::NEEDS_DROP && self.items != 0 {
            // SAFETY: We know for sure that RawTableInner will outlive the
            // returned `RawIter` iterator, and the caller of this function
            // must uphold the safety contract for `drop_elements` method.
            for item in self.iter::<T>() {
                // SAFETY: The caller must uphold the safety contract for
                // `drop_elements` method.
                item.drop();
            }
        }
    }