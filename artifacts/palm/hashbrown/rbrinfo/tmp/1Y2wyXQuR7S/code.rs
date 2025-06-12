unsafe fn bucket_ptr(&self, index: usize, size_of: usize) -> *mut u8 {
        debug_assert_ne!(self.bucket_mask, 0);
        debug_assert!(index < self.buckets());
        let base: *mut u8 = self.data_end().as_ptr();
        base.sub((index + 1) * size_of)
    }