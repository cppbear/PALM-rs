fn size_hint(&self) -> (usize, Option<usize>) {
        // We don't have an item count, so just guess based on the range size.
        let remaining_buckets = if self.end > self.next_ctrl {
            unsafe { offset_from(self.end, self.next_ctrl) }
        } else {
            0
        };

        // Add a group width to include the group we are currently processing.
        (0, Some(Group::WIDTH + remaining_buckets))
    }