pub fn find_mut(&mut self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&mut T> {
        self.raw.get_mut(hash, eq)
    }