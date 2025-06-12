pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
        unsafe {
            let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
            debug_assert!(n <= self.bytes.len());
            let slice = slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
            str::from_utf8_unchecked(slice)
        }
    }