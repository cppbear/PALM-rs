unsafe fn advance_mut(&mut self, cnt: usize) {
        let len = self.len();
        let remaining = self.capacity() - len;

        if remaining < cnt {
            panic_advance(&TryGetError {
                requested: cnt,
                available: remaining,
            });
        }

        // Addition will not overflow since the sum is at most the capacity.
        self.set_len(len + cnt);
    }