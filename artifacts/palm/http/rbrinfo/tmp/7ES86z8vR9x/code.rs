fn drop(&mut self) {
        // Ensure the iterator is consumed
        for _ in self.by_ref() {}

        // All the values have already been yielded out.
        unsafe {
            self.extra_values.set_len(0);
        }
    }