pub unsafe fn replace_bucket_with<F>(&mut self, bucket: Bucket<T>, f: F) -> bool
    where
        F: FnOnce(T) -> Option<T>,
    {
        let index = self.bucket_index(&bucket);
        let old_ctrl = *self.table.ctrl(index);
        debug_assert!(self.is_bucket_full(index));
        let old_growth_left = self.table.growth_left;
        let item = self.remove(bucket).0;
        if let Some(new_item) = f(item) {
            self.table.growth_left = old_growth_left;
            self.table.set_ctrl(index, old_ctrl);
            self.table.items += 1;
            self.bucket(index).write(new_item);
            true
        } else {
            false
        }
    }