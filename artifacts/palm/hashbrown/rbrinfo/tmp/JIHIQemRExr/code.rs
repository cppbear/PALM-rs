unsafe fn fix_insert_slot(&self, mut index: usize) -> InsertSlot {
        // SAFETY: The caller of this function ensures that `index` is in the range `0..=self.bucket_mask`.
        if unlikely(self.is_bucket_full(index)) {
            debug_assert!(self.bucket_mask < Group::WIDTH);
            // SAFETY:
            //
            // * Since the caller of this function ensures that the control bytes are properly
            //   initialized and `ptr = self.ctrl(0)` points to the start of the array of control
            //   bytes, therefore: `ctrl` is valid for reads, properly aligned to `Group::WIDTH`
            //   and points to the properly initialized control bytes (see also
            //   `TableLayout::calculate_layout_for` and `ptr::read`);
            //
            // * Because the caller of this function ensures that the index was provided by the
            //   `self.find_insert_slot_in_group()` function, so for for tables larger than the
            //   group width (self.buckets() >= Group::WIDTH), we will never end up in the given
            //   branch, since `(probe_seq.pos + bit) & self.bucket_mask` in `find_insert_slot_in_group`
            //   cannot return a full bucket index. For tables smaller than the group width, calling
            //   the `unwrap_unchecked` function is also safe, as the trailing control bytes outside
            //   the range of the table are filled with EMPTY bytes (and we know for sure that there
            //   is at least one FULL bucket), so this second scan either finds an empty slot (due to
            //   the load factor) or hits the trailing control bytes (containing EMPTY).
            index = Group::load_aligned(self.ctrl(0))
                .match_empty_or_deleted()
                .lowest_set_bit()
                .unwrap_unchecked();
        }
        InsertSlot { index }
    }