unsafe fn advance_mut(&mut self, cnt: usize) {
        let remaining = self.cap - self.len();
        if cnt > remaining {
            super::panic_advance(&TryGetError {
                requested: cnt,
                available: remaining,
            });
        }
        // Addition won't overflow since it is at most `self.cap`.
        self.len = self.len() + cnt;
    }