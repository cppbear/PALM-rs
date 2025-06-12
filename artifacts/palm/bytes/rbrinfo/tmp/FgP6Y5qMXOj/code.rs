unsafe fn advance_mut(&mut self, cnt: usize) {
        if self.len() < cnt {
            panic_advance(&TryGetError {
                requested: cnt,
                available: self.len(),
            });
        }

        // Lifetime dance taken from `impl Write for &mut [u8]`.
        let (_, b) = core::mem::replace(self, &mut []).split_at_mut(cnt);
        *self = b;
    }