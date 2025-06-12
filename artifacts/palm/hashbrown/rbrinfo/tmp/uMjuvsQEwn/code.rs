pub fn find(&self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&T> {
        self.raw.get(hash, eq)
    }