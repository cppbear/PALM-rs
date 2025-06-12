fn advance(&mut self, cnt: usize) {
        assert!(
            cnt <= self.remaining(),
            "cannot advance past `remaining`: {:?} <= {:?}",
            cnt,
            self.remaining(),
        );
        unsafe {
            // SAFETY: We've checked that `cnt` <= `self.remaining()` and we know that
            // `self.remaining()` <= `self.cap`.
            self.advance_unchecked(cnt);
        }
    }