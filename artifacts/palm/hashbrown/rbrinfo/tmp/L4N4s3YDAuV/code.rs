pub fn get(&self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&T> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.find(hash, eq) {
            Some(bucket) => Some(unsafe { bucket.as_ref() }),
            None => None,
        }
    }