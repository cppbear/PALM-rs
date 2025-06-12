fn next(&mut self) -> Option<usize> {
        // Return if we already yielded all items.
        if self.items == 0 {
            return None;
        }

        let nxt = unsafe {
            // SAFETY:
            // 1. We check number of items to yield using `items` field.
            // 2. The caller ensures that the table is alive and has not moved.
            self.next_impl()
        };

        debug_assert!(nxt.is_some());
        self.items -= 1;

        nxt
    }