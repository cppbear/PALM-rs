fn put_slice(&mut self, src: &[u8]) {
        if self.len() < src.len() {
            panic_advance(&TryGetError {
                requested: src.len(),
                available: self.len(),
            });
        }

        // SAFETY: We just checked that the pointer is valid for `src.len()` bytes.
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr().cast(), src.len());
            self.advance_mut(src.len());
        }
    }