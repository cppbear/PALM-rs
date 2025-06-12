unsafe fn reserve_rehash_inner<A>(
        &mut self,
        alloc: &A,
        additional: usize,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        fallibility: Fallibility,
        layout: TableLayout,
        drop: Option<unsafe fn(*mut u8)>,
    ) -> Result<(), TryReserveError>
    where
        A: Allocator,
    {
        // Avoid `Option::ok_or_else` because it bloats LLVM IR.
        let new_items = match self.items.checked_add(additional) {
            Some(new_items) => new_items,
            None => return Err(fallibility.capacity_overflow()),
        };
        let full_capacity = bucket_mask_to_capacity(self.bucket_mask);
        if new_items <= full_capacity / 2 {
            // Rehash in-place without re-allocating if we have plenty of spare
            // capacity that is locked up due to DELETED entries.

            // SAFETY:
            // 1. We know for sure that `[`RawTableInner`]` has already been allocated
            //    (since new_items <= full_capacity / 2);
            // 2. The caller ensures that `drop` function is the actual drop function of
            //    the elements stored in the table.
            // 3. The caller ensures that `layout` matches the [`TableLayout`] that was
            //    used to allocate this table.
            // 4. The caller ensures that the control bytes of the `RawTableInner`
            //    are already initialized.
            self.rehash_in_place(hasher, layout.size, drop);
            Ok(())
        } else {
            // Otherwise, conservatively resize to at least the next size up
            // to avoid churning deletes into frequent rehashes.
            //
            // SAFETY:
            // 1. We know for sure that `capacity >= self.items`.
            // 2. The caller ensures that `alloc` and `layout` matches the [`Allocator`] and
            //    [`TableLayout`] that were used to allocate this table.
            // 3. The caller ensures that the control bytes of the `RawTableInner`
            //    are already initialized.
            self.resize_inner(
                alloc,
                usize::max(new_items, full_capacity + 1),
                hasher,
                fallibility,
                layout,
            )
        }
    }