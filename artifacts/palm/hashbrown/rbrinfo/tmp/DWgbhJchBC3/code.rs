unsafe fn find_or_find_insert_slot_inner(
        &self,
        hash: u64,
        eq: &mut dyn FnMut(usize) -> bool,
    ) -> Result<usize, InsertSlot> {
        let mut insert_slot = None;

        let tag_hash = Tag::full(hash);
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

            for bit in group.match_tag(tag_hash) {
                let index = (probe_seq.pos + bit) & self.bucket_mask;

                if likely(eq(index)) {
                    return Ok(index);
                }
            }

            // We didn't find the element we were looking for in the group, try to get an
            // insertion slot from the group if we don't have one yet.
            if likely(insert_slot.is_none()) {
                insert_slot = self.find_insert_slot_in_group(&group, &probe_seq);
            }

            // Only stop the search if the group contains at least one empty element.
            // Otherwise, the element that we are looking for might be in a following group.
            if likely(group.match_empty().any_bit_set()) {
                // We must have found a insert slot by now, since the current group contains at
                // least one. For tables smaller than the group width, there will still be an
                // empty element in the current (and only) group due to the load factor.
                unsafe {
                    // SAFETY:
                    // * Caller of this function ensures that the control bytes are properly initialized.
                    //
                    // * We use this function with the slot / index found by `self.find_insert_slot_in_group`
                    return Err(self.fix_insert_slot(insert_slot.unwrap_unchecked()));
                }
            }

            probe_seq.move_next(self.bucket_mask);
        }
    }