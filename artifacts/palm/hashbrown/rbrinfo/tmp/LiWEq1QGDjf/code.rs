unsafe fn next_impl(&mut self) -> Option<usize> {
        loop {
            if let Some(index) = self.current_group.next() {
                // The returned `self.group_first_index + index` will always
                // be in the range `0..self.buckets()`. See explanation below.
                return Some(self.group_first_index + index);
            }

            // SAFETY: The caller of this function ensures that:
            //
            // 1. It never tries to iterate after getting all the elements;
            // 2. The table is alive and did not moved;
            // 3. The first `self.ctrl` pointed to the start of the array of control bytes.
            //
            // Taking the above into account, we always stay within the bounds, because:
            //
            // 1. For tables smaller than the group width (self.buckets() <= Group::WIDTH),
            //    we will never end up in the given branch, since we should have already
            //    yielded all the elements of the table.
            //
            // 2. For tables larger than the group width. The number of buckets is a
            //    power of two (2 ^ n), Group::WIDTH is also power of two (2 ^ k). Since
            //    `(2 ^ n) > (2 ^ k)`, than `(2 ^ n) % (2 ^ k) = 0`. As we start from the
            //    the start of the array of control bytes, and never try to iterate after
            //    getting all the elements, the last `self.ctrl` will be equal to
            //    the `self.buckets() - Group::WIDTH`, so `self.current_group.next()`
            //    will always contains indices within the range `0..Group::WIDTH`,
            //    and subsequent `self.group_first_index + index` will always return a
            //    number less than `self.buckets()`.
            self.ctrl = NonNull::new_unchecked(self.ctrl.as_ptr().add(Group::WIDTH));

            // SAFETY: See explanation above.
            self.current_group = Group::load_aligned(self.ctrl.as_ptr().cast())
                .match_full()
                .into_iter();
            self.group_first_index += Group::WIDTH;
        }
    }