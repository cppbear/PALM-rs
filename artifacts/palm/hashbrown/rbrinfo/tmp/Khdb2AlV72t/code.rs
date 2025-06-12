unsafe fn free_buckets<A>(&mut self, alloc: &A, table_layout: TableLayout)
    where
        A: Allocator,
    {
        // SAFETY: The caller must uphold the safety contract for `free_buckets`
        // method.
        let (ptr, layout) = self.allocation_info(table_layout);
        alloc.deallocate(ptr, layout);
    }