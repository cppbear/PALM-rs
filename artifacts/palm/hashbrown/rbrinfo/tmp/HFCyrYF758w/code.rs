pub unsafe fn is_bucket_full(&self, index: usize) -> bool {
        self.table.is_bucket_full(index)
    }