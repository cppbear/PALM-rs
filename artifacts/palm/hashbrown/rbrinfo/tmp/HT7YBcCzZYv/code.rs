pub fn find_or_find_insert_slot(
        &mut self,
        hash: u64,
        mut eq: impl FnMut(&T) -> bool,
        hasher: impl Fn(&T) -> u64,
    ) -> Result<Bucket<T>, InsertSlot> {
        self.reserve(1, hasher);

        unsafe {
            // SAFETY:
            // 1. We know for sure that there is at least one empty `bucket` in the table.
            // 2. The [`RawTableInner`] must already have properly initialized control bytes since we will
            //    never expose `RawTable::new_uninitialized` in a public API.
            // 3. The `find_or_find_insert_slot_inner` function returns the `index` of only the full bucket,
            //    which is in the range `0..self.buckets()` (since there is at least one empty `bucket` in
            //    the table), so calling `self.bucket(index)` and `Bucket::as_ref` is safe.
            match self
                .table
                .find_or_find_insert_slot_inner(hash, &mut |index| eq(self.bucket(index).as_ref()))
            {
                // SAFETY: See explanation above.
                Ok(index) => Ok(self.bucket(index)),
                Err(slot) => Err(slot),
            }
        }
    }