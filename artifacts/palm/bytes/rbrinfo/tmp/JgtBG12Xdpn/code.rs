fn put_slice(&mut self, src: &[u8]) {
        if self.len() < src.len() {
            panic_advance(&TryGetError {
                requested: src.len(),
                available: self.len(),
            });
        }

        self[..src.len()].copy_from_slice(src);
        // SAFETY: We just initialized `src.len()` bytes.
        unsafe { self.advance_mut(src.len()) };
    }