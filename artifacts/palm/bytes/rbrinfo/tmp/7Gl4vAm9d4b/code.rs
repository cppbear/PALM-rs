pub fn resize(&mut self, new_len: usize, value: u8) {
        let additional = if let Some(additional) = new_len.checked_sub(self.len()) {
            additional
        } else {
            self.truncate(new_len);
            return;
        };

        if additional == 0 {
            return;
        }

        self.reserve(additional);
        let dst = self.spare_capacity_mut().as_mut_ptr();
        // SAFETY: `spare_capacity_mut` returns a valid, properly aligned pointer and we've
        // reserved enough space to write `additional` bytes.
        unsafe { ptr::write_bytes(dst, value, additional) };

        // SAFETY: There are at least `new_len` initialized bytes in the buffer so no
        // uninitialized bytes are being exposed.
        unsafe { self.set_len(new_len) };
    }