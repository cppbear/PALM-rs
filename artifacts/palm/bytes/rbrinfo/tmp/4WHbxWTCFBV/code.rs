fn put_bytes(&mut self, val: u8, cnt: usize) {
        self.reserve(cnt);
        unsafe {
            let dst = self.spare_capacity_mut();
            // Reserved above
            debug_assert!(dst.len() >= cnt);

            ptr::write_bytes(dst.as_mut_ptr(), val, cnt);

            self.advance_mut(cnt);
        }
    }