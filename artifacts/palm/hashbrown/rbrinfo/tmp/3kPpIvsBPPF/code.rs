unsafe fn drop_inner_table<T, A: Allocator>(&mut self, alloc: &A, table_layout: TableLayout) {
        if !self.is_empty_singleton() {
            unsafe {
                // SAFETY: The caller must uphold the safety contract for `drop_inner_table` method.
                self.drop_elements::<T>();
                // SAFETY:
                // 1. We have checked that our table is allocated.
                // 2. The caller must uphold the safety contract for `drop_inner_table` method.
                self.free_buckets(alloc, table_layout);
            }
        }
    }