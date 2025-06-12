fn fallible_with_capacity<A>(
        alloc: &A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -> Result<Self, TryReserveError>
    where
        A: Allocator,
    {
        if capacity == 0 {
            Ok(Self::NEW)
        } else {
            // SAFETY: We checked that we could successfully allocate the new table, and then
            // initialized all control bytes with the constant `Tag::EMPTY` byte.
            unsafe {
                let buckets =
                    capacity_to_buckets(capacity).ok_or_else(|| fallibility.capacity_overflow())?;

                let result = Self::new_uninitialized(alloc, table_layout, buckets, fallibility)?;
                // SAFETY: We checked that the table is allocated and therefore the table already has
                // `self.bucket_mask + 1 + Group::WIDTH` number of control bytes (see TableLayout::calculate_layout_for)
                // so writing `self.num_ctrl_bytes() == bucket_mask + 1 + Group::WIDTH` bytes is safe.
                result
                    .ctrl(0)
                    .write_bytes(Tag::EMPTY.0, result.num_ctrl_bytes());

                Ok(result)
            }
        }
    }