unsafe fn erase(&mut self, index: usize) {
        debug_assert!(self.is_bucket_full(index));

        // This is the same as `index.wrapping_sub(Group::WIDTH) % self.buckets()` because
        // the number of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
        let index_before = index.wrapping_sub(Group::WIDTH) & self.bucket_mask;
        // SAFETY:
        // - The caller must uphold the safety contract for `erase` method;
        // - `index_before` is guaranteed to be in range due to masking with `self.bucket_mask`
        let empty_before = Group::load(self.ctrl(index_before)).match_empty();
        let empty_after = Group::load(self.ctrl(index)).match_empty();

        // Inserting and searching in the map is performed by two key functions:
        //
        // - The `find_insert_slot` function that looks up the index of any `Tag::EMPTY` or `Tag::DELETED`
        //   slot in a group to be able to insert. If it doesn't find an `Tag::EMPTY` or `Tag::DELETED`
        //   slot immediately in the first group, it jumps to the next `Group` looking for it,
        //   and so on until it has gone through all the groups in the control bytes.
        //
        // - The `find_inner` function that looks for the index of the desired element by looking
        //   at all the `FULL` bytes in the group. If it did not find the element right away, and
        //   there is no `Tag::EMPTY` byte in the group, then this means that the `find_insert_slot`
        //   function may have found a suitable slot in the next group. Therefore, `find_inner`
        //   jumps further, and if it does not find the desired element and again there is no `Tag::EMPTY`
        //   byte, then it jumps further, and so on. The search stops only if `find_inner` function
        //   finds the desired element or hits an `Tag::EMPTY` slot/byte.
        //
        // Accordingly, this leads to two consequences:
        //
        // - The map must have `Tag::EMPTY` slots (bytes);
        //
        // - You can't just mark the byte to be erased as `Tag::EMPTY`, because otherwise the `find_inner`
        //   function may stumble upon an `Tag::EMPTY` byte before finding the desired element and stop
        //   searching.
        //
        // Thus it is necessary to check all bytes after and before the erased element. If we are in
        // a contiguous `Group` of `FULL` or `Tag::DELETED` bytes (the number of `FULL` or `Tag::DELETED` bytes
        // before and after is greater than or equal to `Group::WIDTH`), then we must mark our byte as
        // `Tag::DELETED` in order for the `find_inner` function to go further. On the other hand, if there
        // is at least one `Tag::EMPTY` slot in the `Group`, then the `find_inner` function will still stumble
        // upon an `Tag::EMPTY` byte, so we can safely mark our erased byte as `Tag::EMPTY` as well.
        //
        // Finally, since `index_before == (index.wrapping_sub(Group::WIDTH) & self.bucket_mask) == index`
        // and given all of the above, tables smaller than the group width (self.buckets() < Group::WIDTH)
        // cannot have `Tag::DELETED` bytes.
        //
        // Note that in this context `leading_zeros` refers to the bytes at the end of a group, while
        // `trailing_zeros` refers to the bytes at the beginning of a group.
        let ctrl = if empty_before.leading_zeros() + empty_after.trailing_zeros() >= Group::WIDTH {
            Tag::DELETED
        } else {
            self.growth_left += 1;
            Tag::EMPTY
        };
        // SAFETY: the caller must uphold the safety contract for `erase` method.
        self.set_ctrl(index, ctrl);
        self.items -= 1;
    }