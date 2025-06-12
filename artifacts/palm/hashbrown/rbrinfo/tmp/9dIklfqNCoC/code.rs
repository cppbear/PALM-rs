unsafe fn find_inner(&self, hash: u64, eq: &mut dyn FnMut(usize) -> bool) -> Option<usize> {
        let tag_hash = Tag::full(hash);
        let mut probe_seq = self.probe_seq(hash);

        loop {
            // SAFETY:
            // * Caller of this function ensures that the control bytes are properly initialized.
            //
            // * `ProbeSeq.pos` cannot be greater than `self.bucket_mask = self.buckets() - 1`
            //   of the table due to masking with `self.bucket_mask`.
            //
            // * Even if `ProbeSeq.pos` returns `position == self.bucket_mask`, it is safe to
            //   call `Group::load` due to the extended control bytes range, which is
            //  `self.bucket_mask + 1 + Group::WIDTH` (in fact, this means that the last control
            //   byte will never be read for the allocated table);
            //
            // * Also, even if `RawTableInner` is not already allocated, `ProbeSeq.pos` will
            //   always return "0" (zero), so Group::load will read unaligned `Group::static_empty()`
            //   bytes, which is safe (see RawTableInner::new_in).
            let group = unsafe { Group::load(self.ctrl(probe_seq.pos)) };

            for bit in group.match_tag(tag_hash) {
                // This is the same as `(probe_seq.pos + bit) % self.buckets()` because the number
                // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
                let index = (probe_seq.pos + bit) & self.bucket_mask;

                if likely(eq(index)) {
                    return Some(index);
                }
            }

            if likely(group.match_empty().any_bit_set()) {
                return None;
            }

            probe_seq.move_next(self.bucket_mask);
        }
    }