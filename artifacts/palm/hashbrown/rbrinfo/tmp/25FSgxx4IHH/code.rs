unsafe fn erase_no_drop(&mut self, item: &Bucket<T>) {
        let index = self.bucket_index(item);
        self.table.erase(index);
    }