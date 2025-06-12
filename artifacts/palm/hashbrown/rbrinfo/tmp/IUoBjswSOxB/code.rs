pub fn find(&self, hash: u64, mut eq: impl FnMut(&T) -> bool) -> Option<Bucket<T>> {
        unsafe {
            // SAFETY:
            // 1. The [`RawTableInner`] must already have properly initialized control bytes since we
            //    will never expose `RawTable::new_uninitialized` in a public API.
            // 1. The `find_inner` function returns the `index` of only the full bucket, which is in
            //    the range `0..self.buckets()`, so calling `self.bucket(index)` and `Bucket::as_ref`
            //    is safe.
            let result = self
                .table
                .find_inner(hash, &mut |index| eq(self.bucket(index).as_ref()));

            // Avoid `Option::map` because it bloats LLVM IR.
            match result {
                // SAFETY: See explanation above.
                Some(index) => Some(self.bucket(index)),
                None => None,
            }
        }
    }