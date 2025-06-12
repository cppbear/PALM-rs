pub fn insert(&mut self, hash: u64, value: T, hasher: impl Fn(&T) -> u64) -> Bucket<T> {
        unsafe {
            // SAFETY:
            // 1. The [`RawTableInner`] must already have properly initialized control bytes since
            //    we will never expose `RawTable::new_uninitialized` in a public API.
            //
            // 2. We reserve additional space (if necessary) right after calling this function.
            let mut slot = self.table.find_insert_slot(hash);

            // We can avoid growing the table once we have reached our load factor if we are replacing
            // a tombstone. This works since the number of EMPTY slots does not change in this case.
            //
            // SAFETY: The function is guaranteed to return [`InsertSlot`] that contains an index
            // in the range `0..=self.buckets()`.
            let old_ctrl = *self.table.ctrl(slot.index);
            if unlikely(self.table.growth_left == 0 && old_ctrl.special_is_empty()) {
                self.reserve(1, hasher);
                // SAFETY: We know for sure that `RawTableInner` has control bytes
                // initialized and that there is extra space in the table.
                slot = self.table.find_insert_slot(hash);
            }

            self.insert_in_slot(hash, slot, value)
        }
    }