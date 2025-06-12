unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(&mut self) -> Option<Bucket<T>> {
        loop {
            if let Some(index) = self.current_group.next() {
                return Some(self.data.next_n(index));
            }

            if DO_CHECK_PTR_RANGE && self.next_ctrl >= self.end {
                return None;
            }

            // We might read past self.end up to the next group boundary,
            // but this is fine because it only occurs on tables smaller
            // than the group size where the trailing control bytes are all
            // EMPTY. On larger tables self.end is guaranteed to be aligned
            // to the group size (since tables are power-of-two sized).
            self.current_group = Group::load_aligned(self.next_ctrl.cast())
                .match_full()
                .into_iter();
            self.data = self.data.next_n(Group::WIDTH);
            self.next_ctrl = self.next_ctrl.add(Group::WIDTH);
        }
    }