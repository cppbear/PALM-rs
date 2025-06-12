unsafe fn set_ctrl(&mut self, index: usize, ctrl: Tag) {
        // Replicate the first Group::WIDTH control bytes at the end of
        // the array without using a branch. If the tables smaller than
        // the group width (self.buckets() < Group::WIDTH),
        // `index2 = Group::WIDTH + index`, otherwise `index2` is:
        //
        // - If index >= Group::WIDTH then index == index2.
        // - Otherwise index2 == self.bucket_mask + 1 + index.
        //
        // The very last replicated control byte is never actually read because
        // we mask the initial index for unaligned loads, but we write it
        // anyways because it makes the set_ctrl implementation simpler.
        //
        // If there are fewer buckets than Group::WIDTH then this code will
        // replicate the buckets at the end of the trailing group. For example
        // with 2 buckets and a group size of 4, the control bytes will look
        // like this:
        //
        //     Real    |             Replicated
        // ---------------------------------------------
        // | [A] | [B] | [Tag::EMPTY] | [EMPTY] | [A] | [B] |
        // ---------------------------------------------

        // This is the same as `(index.wrapping_sub(Group::WIDTH)) % self.buckets() + Group::WIDTH`
        // because the number of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
        let index2 = ((index.wrapping_sub(Group::WIDTH)) & self.bucket_mask) + Group::WIDTH;

        // SAFETY: The caller must uphold the safety rules for the [`RawTableInner::set_ctrl`]
        *self.ctrl(index) = ctrl;
        *self.ctrl(index2) = ctrl;
    }