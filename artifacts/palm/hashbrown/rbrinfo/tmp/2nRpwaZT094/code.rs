pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self {
            table: RawTableInner::with_capacity(&alloc, Self::TABLE_LAYOUT, capacity),
            alloc,
            marker: PhantomData,
        }
    }