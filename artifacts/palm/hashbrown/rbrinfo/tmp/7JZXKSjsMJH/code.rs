fn next(&mut self) -> Option<Bucket<T>> {
        unsafe {
            match self.inner.next() {
                Some(index) => {
                    // Can't use `RawTable::bucket` here as we don't have
                    // an actual `RawTable` reference to use.
                    debug_assert!(index <= self.inner.bucket_mask);
                    let bucket = Bucket::from_base_index(self.inner.ctrl.cast(), index);
                    Some(bucket)
                }
                None => None,
            }
        }
    }