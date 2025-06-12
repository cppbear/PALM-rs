unsafe fn prepare_rehash_in_place(&mut self) {
        // Bulk convert all full control bytes to DELETED, and all DELETED control bytes to EMPTY.
        // This effectively frees up all buckets containing a DELETED entry.
        //
        // SAFETY:
        // 1. `i` is guaranteed to be within bounds since we are iterating from zero to `buckets - 1`;
        // 2. Even if `i` will be `i == self.bucket_mask`, it is safe to call `Group::load_aligned`
        //    due to the extended control bytes range, which is `self.bucket_mask + 1 + Group::WIDTH`;
        // 3. The caller of this function guarantees that [`RawTableInner`] has already been allocated;
        // 4. We can use `Group::load_aligned` and `Group::store_aligned` here since we start from 0
        //    and go to the end with a step equal to `Group::WIDTH` (see TableLayout::calculate_layout_for).
        for i in (0..self.buckets()).step_by(Group::WIDTH) {
            let group = Group::load_aligned(self.ctrl(i));
            let group = group.convert_special_to_empty_and_full_to_deleted();
            group.store_aligned(self.ctrl(i));
        }

        // Fix up the trailing control bytes. See the comments in set_ctrl
        // for the handling of tables smaller than the group width.
        //
        // SAFETY: The caller of this function guarantees that [`RawTableInner`]
        // has already been allocated
        if unlikely(self.buckets() < Group::WIDTH) {
            // SAFETY: We have `self.bucket_mask + 1 + Group::WIDTH` number of control bytes,
            // so copying `self.buckets() == self.bucket_mask + 1` bytes with offset equal to
            // `Group::WIDTH` is safe
            self.ctrl(0)
                .copy_to(self.ctrl(Group::WIDTH), self.buckets());
        } else {
            // SAFETY: We have `self.bucket_mask + 1 + Group::WIDTH` number of
            // control bytes,so copying `Group::WIDTH` bytes with offset equal
            // to `self.buckets() == self.bucket_mask + 1` is safe
            self.ctrl(0)
                .copy_to(self.ctrl(self.buckets()), Group::WIDTH);
        }
    }