pub unsafe fn remove(&mut self, item: Bucket<T>) -> (T, InsertSlot) {
        self.erase_no_drop(&item);
        (
            item.read(),
            InsertSlot {
                index: self.bucket_index(&item),
            },
        )
    }