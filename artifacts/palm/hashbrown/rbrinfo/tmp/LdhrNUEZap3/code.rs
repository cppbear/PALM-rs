unsafe fn fold_impl<F, B>(mut self, mut n: usize, mut acc: B, mut f: F) -> B
    where
        F: FnMut(B, Bucket<T>) -> B,
    {
        loop {
            while let Some(index) = self.current_group.next() {
                // The returned `index` will always be in the range `0..Group::WIDTH`,
                // so that calling `self.data.next_n(index)` is safe (see detailed explanation below).
                debug_assert!(n != 0);
                let bucket = self.data.next_n(index);
                acc = f(acc, bucket);
                n -= 1;
            }

            if n == 0 {
                return acc;
            }

            // SAFETY: The caller of this function ensures that:
            //
            // 1. The provided `n` value matches the actual number of items in the table;
            // 2. The table is alive and did not moved.
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
            //    start of the array of control bytes, and never try to iterate after
            //    getting all the elements, the last `self.current_group` will read bytes
            //    from the `self.buckets() - Group::WIDTH` index.  We know also that
            //    `self.current_group.next()` will always return indices within the range
            //    `0..Group::WIDTH`.
            //
            //    Knowing all of the above and taking into account that we are synchronizing
            //    the `self.data` index with the index we used to read the `self.current_group`,
            //    the subsequent `self.data.next_n(index)` will always return a bucket with
            //    an index number less than `self.buckets()`.
            //
            //    The last `self.next_ctrl`, whose index would be `self.buckets()`, will never
            //    actually be read, since we should have already yielded all the elements of
            //    the table.
            self.current_group = Group::load_aligned(self.next_ctrl.cast())
                .match_full()
                .into_iter();
            self.data = self.data.next_n(Group::WIDTH);
            self.next_ctrl = self.next_ctrl.add(Group::WIDTH);
        }
    }