pub fn shrink_to(&mut self, min_size: usize, hasher: impl Fn(&T) -> u64) {
        // Calculate the minimal number of elements that we need to reserve
        // space for.
        let min_size = usize::max(self.table.items, min_size);
        if min_size == 0 {
            let mut old_inner = mem::replace(&mut self.table, RawTableInner::NEW);
            unsafe {
                // SAFETY:
                // 1. We call the function only once;
                // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
                //    and [`TableLayout`] that were used to allocate this table.
                // 3. If any elements' drop function panics, then there will only be a memory leak,
                //    because we have replaced the inner table with a new one.
                old_inner.drop_inner_table::<T, _>(&self.alloc, Self::TABLE_LAYOUT);
            }
            return;
        }

        // Calculate the number of buckets that we need for this number of
        // elements. If the calculation overflows then the requested bucket
        // count must be larger than what we have right and nothing needs to be
        // done.
        let min_buckets = match capacity_to_buckets(min_size) {
            Some(buckets) => buckets,
            None => return,
        };

        // If we have more buckets than we need, shrink the table.
        if min_buckets < self.buckets() {
            // Fast path if the table is empty
            if self.table.items == 0 {
                let new_inner =
                    RawTableInner::with_capacity(&self.alloc, Self::TABLE_LAYOUT, min_size);
                let mut old_inner = mem::replace(&mut self.table, new_inner);
                unsafe {
                    // SAFETY:
                    // 1. We call the function only once;
                    // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
                    //    and [`TableLayout`] that were used to allocate this table.
                    // 3. If any elements' drop function panics, then there will only be a memory leak,
                    //    because we have replaced the inner table with a new one.
                    old_inner.drop_inner_table::<T, _>(&self.alloc, Self::TABLE_LAYOUT);
                }
            } else {
                // Avoid `Result::unwrap_or_else` because it bloats LLVM IR.
                unsafe {
                    // SAFETY:
                    // 1. We know for sure that `min_size >= self.table.items`.
                    // 2. The [`RawTableInner`] must already have properly initialized control bytes since
                    //    we will never expose RawTable::new_uninitialized in a public API.
                    if self
                        .resize(min_size, hasher, Fallibility::Infallible)
                        .is_err()
                    {
                        // SAFETY: The result of calling the `resize` function cannot be an error
                        // because `fallibility == Fallibility::Infallible.
                        hint::unreachable_unchecked()
                    }
                }
            }
        }
    }