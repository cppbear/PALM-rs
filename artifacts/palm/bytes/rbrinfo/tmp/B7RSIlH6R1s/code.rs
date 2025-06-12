pub fn split_off(&mut self, at: usize) -> BytesMut {
        assert!(
            at <= self.capacity(),
            "split_off out of bounds: {:?} <= {:?}",
            at,
            self.capacity(),
        );
        unsafe {
            let mut other = self.shallow_clone();
            // SAFETY: We've checked that `at` <= `self.capacity()` above.
            other.advance_unchecked(at);
            self.cap = at;
            self.len = cmp::min(self.len, at);
            other
        }
    }