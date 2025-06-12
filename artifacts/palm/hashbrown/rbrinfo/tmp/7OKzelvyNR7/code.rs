unsafe fn find_insert_slot(&self, hash: u64) -> InsertSlot {
        let mut probe_seq = self.probe_seq(hash);
        loop {
            // SAFETY:
            // * Caller of this function ensures that the control bytes are properly initialized.
            //
            // * `ProbeSeq.pos` cannot be greater than `self.bucket_mask = self.buckets() - 1`
            //   of the table due to masking with `self.bucket_mask` and also because the number
            //   of buckets is a power of two (see `self.probe_seq` function).
            //
            // * Even if `ProbeSeq.pos` returns `position == self.bucket_mask`, it is safe to
            //   call `Group::load` due to the extended control bytes range, which is
            //  `self.bucket_mask + 1 + Group::WIDTH` (in fact, this means that the last control
            //   byte will never be read for the allocated table);
            //
            // * Also, even if `RawTableInner` is not already allocated, `ProbeSeq.pos` will
            //   always return "0" (zero), so Group::load will read unaligned `Group::static_empty()`
            //   bytes, which is safe (see RawTableInner::new).
            let group = unsafe { Group::load(self.ctrl(probe_seq.pos)) };

            let index = self.find_insert_slot_in_group(&group, &probe_seq);
            if likely(index.is_some()) {
                // SAFETY:
                // * Caller of this function ensures that the control bytes are properly initialized.
                //
                // * We use this function with the slot / index found by `self.find_insert_slot_in_group`
                unsafe {
                    return self.fix_insert_slot(index.unwrap_unchecked());
                }
            }
            probe_seq.move_next(self.bucket_mask);
        }
    }