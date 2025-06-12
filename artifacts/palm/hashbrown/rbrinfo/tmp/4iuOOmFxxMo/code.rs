unsafe fn resize_inner<A>(
        &mut self,
        alloc: &A,
        capacity: usize,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        fallibility: Fallibility,
        layout: TableLayout,
    ) -> Result<(), TryReserveError>
    where
        A: Allocator,
    {
        // SAFETY: We know for sure that `alloc` and `layout` matches the [`Allocator`] and [`TableLayout`]
        // that were used to allocate this table.
        let mut new_table = self.prepare_resize(alloc, layout, capacity, fallibility)?;

        // SAFETY: We know for sure that RawTableInner will outlive the
        // returned `FullBucketsIndices` iterator, and the caller of this
        // function ensures that the control bytes are properly initialized.
        for full_byte_index in self.full_buckets_indices() {
            // This may panic.
            let hash = hasher(self, full_byte_index);

            // SAFETY:
            // We can use a simpler version of insert() here since:
            // 1. There are no DELETED entries.
            // 2. We know there is enough space in the table.
            // 3. All elements are unique.
            // 4. The caller of this function guarantees that `capacity > 0`
            //    so `new_table` must already have some allocated memory.
            // 5. We set `growth_left` and `items` fields of the new table
            //    after the loop.
            // 6. We insert into the table, at the returned index, the data
            //    matching the given hash immediately after calling this function.
            let (new_index, _) = new_table.prepare_insert_slot(hash);

            // SAFETY:
            //
            // * `src` is valid for reads of `layout.size` bytes, since the
            //   table is alive and the `full_byte_index` is guaranteed to be
            //   within bounds (see `FullBucketsIndices::next_impl`);
            //
            // * `dst` is valid for writes of `layout.size` bytes, since the
            //   caller ensures that `table_layout` matches the [`TableLayout`]
            //   that was used to allocate old table and we have the `new_index`
            //   returned by `prepare_insert_slot`.
            //
            // * Both `src` and `dst` are properly aligned.
            //
            // * Both `src` and `dst` point to different region of memory.
            ptr::copy_nonoverlapping(
                self.bucket_ptr(full_byte_index, layout.size),
                new_table.bucket_ptr(new_index, layout.size),
                layout.size,
            );
        }

        // The hash function didn't panic, so we can safely set the
        // `growth_left` and `items` fields of the new table.
        new_table.growth_left -= self.items;
        new_table.items = self.items;

        // We successfully copied all elements without panicking. Now replace
        // self with the new table. The old table will have its memory freed but
        // the items will not be dropped (since they have been moved into the
        // new table).
        // SAFETY: The caller ensures that `table_layout` matches the [`TableLayout`]
        // that was used to allocate this table.
        mem::swap(self, &mut new_table);

        Ok(())
    }