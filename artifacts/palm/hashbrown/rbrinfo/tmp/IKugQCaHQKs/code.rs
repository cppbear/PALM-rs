pub fn get_mut(&mut self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&mut T> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.find(hash, eq) {
            Some(bucket) => Some(unsafe { bucket.as_mut() }),
            None => None,
        }
    }