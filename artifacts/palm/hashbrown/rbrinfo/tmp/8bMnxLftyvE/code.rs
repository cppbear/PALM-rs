unsafe fn is_bucket_full(&self, index: usize) -> bool {
        debug_assert!(index < self.buckets());
        (*self.ctrl(index)).is_full()
    }