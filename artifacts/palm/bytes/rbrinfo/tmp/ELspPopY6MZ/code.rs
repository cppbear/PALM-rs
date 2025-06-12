pub fn copy_from_slice(&mut self, src: &[u8]) {
        use core::ptr;

        assert_eq!(self.len(), src.len());

        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), self.len());
        }
    }