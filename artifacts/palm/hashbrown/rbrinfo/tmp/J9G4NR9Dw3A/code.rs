fn prepare_resize<'a, A>(
        &self,
        alloc: &'a A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -> Result<crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self) + 'a>, TryReserveError>
    where
        A: Allocator,
    {
        debug_assert!(self.items <= capacity);

        // Allocate and initialize the new table.
        let new_table =
            RawTableInner::fallible_with_capacity(alloc, table_layout, capacity, fallibility)?;

        // The hash function may panic, in which case we simply free the new
        // table without dropping any elements that may have been copied into
        // it.
        //
        // This guard is also used to free the old table on success, see
        // the comment at the bottom of this function.
        Ok(guard(new_table, move |self_| {
            if !self_.is_empty_singleton() {
                // SAFETY:
                // 1. We have checked that our table is allocated.
                // 2. We know for sure that the `alloc` and `table_layout` matches the
                //    [`Allocator`] and [`TableLayout`] used to allocate this table.
                unsafe { self_.free_buckets(alloc, table_layout) };
            }
        }))
    }