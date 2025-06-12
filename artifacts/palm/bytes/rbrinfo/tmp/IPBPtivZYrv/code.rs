fn copy_to_slice(&mut self, dst: &mut [u8]) {
        if self.len() < dst.len() {
            panic_advance(&TryGetError {
                requested: dst.len(),
                available: self.len(),
            });
        }

        dst.copy_from_slice(&self[..dst.len()]);
        self.advance(dst.len());
    }