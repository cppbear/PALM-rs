fn next(&mut self) -> Option<Self::Item> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some(bucket) => Some(unsafe { bucket.as_mut() }),
            None => None,
        }
    }