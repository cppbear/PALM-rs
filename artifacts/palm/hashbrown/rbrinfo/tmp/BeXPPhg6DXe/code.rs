pub fn allocation_size(&self) -> usize {
        // SAFETY: We use the same `table_layout` that was used to allocate
        // this table.
        unsafe { self.table.allocation_size_or_zero(Self::TABLE_LAYOUT) }
    }