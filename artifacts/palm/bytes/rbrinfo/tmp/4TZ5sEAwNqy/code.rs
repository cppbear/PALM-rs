pub fn clear(&mut self) {
        // SAFETY: Setting the length to zero cannot expose uninitialized bytes.
        unsafe { self.set_len(0) };
    }