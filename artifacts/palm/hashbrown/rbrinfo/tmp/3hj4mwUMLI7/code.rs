unsafe fn allocation_size_or_zero(&self, table_layout: TableLayout) -> usize {
        if self.is_empty_singleton() {
            0
        } else {
            // SAFETY:
            // 1. We have checked that our table is allocated.
            // 2. The caller ensures that `table_layout` matches the [`TableLayout`]
            // that was used to allocate this table.
            unsafe { self.allocation_info(table_layout).1.size() }
        }
    }