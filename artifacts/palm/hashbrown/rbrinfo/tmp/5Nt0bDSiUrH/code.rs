fn next(&mut self) -> Option<Bucket<T>> {
        // Inner iterator iterates over buckets
        // so it can do unnecessary work if we already yielded all items.
        if self.items == 0 {
            return None;
        }

        let nxt = unsafe {
            // SAFETY: We check number of items to yield using `items` field.
            self.iter.next_impl::<false>()
        };

        debug_assert!(nxt.is_some());
        self.items -= 1;

        nxt
    }