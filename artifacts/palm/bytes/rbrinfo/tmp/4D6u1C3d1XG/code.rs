pub fn split_to(&mut self, at: usize) -> BytesMut {
        assert!(
            at <= self.len(),
            "split_to out of bounds: {:?} <= {:?}",
            at,
            self.len(),
        );

        unsafe {
            let mut other = self.shallow_clone();
            // SAFETY: We've checked that `at` <= `self.len()` and we know that `self.len()` <=
            // `self.capacity()`.
            self.advance_unchecked(at);
            other.cap = at;
            other.len = at;
            other
        }
    }