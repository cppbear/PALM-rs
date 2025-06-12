pub fn buckets(&self) -> usize {
        self.table.bucket_mask + 1
    }