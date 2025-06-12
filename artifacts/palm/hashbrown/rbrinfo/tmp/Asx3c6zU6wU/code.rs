fn drop(&mut self) {
        unsafe {
            // SAFETY:
            // 1. We call the function only once;
            // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
            //    and [`TableLayout`] that were used to allocate this table.
            // 3. If the drop function of any elements fails, then only a memory leak will occur,
            //    and we don't care because we are inside the `Drop` function of the `RawTable`,
            //    so there won't be any table left in an inconsistent state.
            self.table
                .drop_inner_table::<T, _>(&self.alloc, Self::TABLE_LAYOUT);
        }
    }