fn put_bytes(&mut self, val: u8, cnt: usize) {
        if self.len() < cnt {
            panic_advance(&TryGetError {
                requested: cnt,
                available: self.len(),
            });
        }

        // SAFETY: We just checked that the pointer is valid for `cnt` bytes.
        unsafe {
            ptr::write_bytes(self.as_mut_ptr() as *mut u8, val, cnt);
            self.advance_mut(cnt);
        }
    }