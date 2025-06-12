fn clone(&self) -> Self {
        if self.table.is_empty_singleton() {
            Self::new_in(self.alloc.clone())
        } else {
            unsafe {
                // Avoid `Result::ok_or_else` because it bloats LLVM IR.
                //
                // SAFETY: This is safe as we are taking the size of an already allocated table
                // and therefore capacity overflow cannot occur, `self.table.buckets()` is power
                // of two and all allocator errors will be caught inside `RawTableInner::new_uninitialized`.
                let mut new_table = match Self::new_uninitialized(
                    self.alloc.clone(),
                    self.table.buckets(),
                    Fallibility::Infallible,
                ) {
                    Ok(table) => table,
                    Err(_) => hint::unreachable_unchecked(),
                };

                // Cloning elements may fail (the clone function may panic). But we don't
                // need to worry about uninitialized control bits, since:
                // 1. The number of items (elements) in the table is zero, which means that
                //    the control bits will not be read by Drop function.
                // 2. The `clone_from_spec` method will first copy all control bits from
                //    `self` (thus initializing them). But this will not affect the `Drop`
                //    function, since the `clone_from_spec` function sets `items` only after
                //    successfully cloning all elements.
                new_table.clone_from_spec(self);
                new_table
            }
        }
    }