unsafe fn resize(
        &mut self,
        capacity: usize,
        hasher: impl Fn(&T) -> u64,
        fallibility: Fallibility,
    ) -> Result<(), TryReserveError> {
        // SAFETY:
        // 1. The caller of this function guarantees that `capacity >= self.table.items`.
        // 2. We know for sure that `alloc` and `layout` matches the [`Allocator`] and
        //    [`TableLayout`] that were used to allocate this table.
        // 3. The caller ensures that the control bytes of the `RawTableInner`
        //    are already initialized.
        self.table.resize_inner(
            &self.alloc,
            capacity,
            &|table, index| hasher(table.bucket::<T>(index).as_ref()),
            fallibility,
            Self::TABLE_LAYOUT,
        )
    }