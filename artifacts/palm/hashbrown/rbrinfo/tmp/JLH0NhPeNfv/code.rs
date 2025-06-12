unsafe fn reserve_rehash(
        &mut self,
        additional: usize,
        hasher: impl Fn(&T) -> u64,
        fallibility: Fallibility,
    ) -> Result<(), TryReserveError> {
        unsafe {
            // SAFETY:
            // 1. We know for sure that `alloc` and `layout` matches the [`Allocator`] and
            //    [`TableLayout`] that were used to allocate this table.
            // 2. The `drop` function is the actual drop function of the elements stored in
            //    the table.
            // 3. The caller ensures that the control bytes of the `RawTableInner`
            //    are already initialized.
            self.table.reserve_rehash_inner(
                &self.alloc,
                additional,
                &|table, index| hasher(table.bucket::<T>(index).as_ref()),
                fallibility,
                Self::TABLE_LAYOUT,
                if T::NEEDS_DROP {
                    Some(|ptr| ptr::drop_in_place(ptr as *mut T))
                } else {
                    None
                },
            )
        }
    }