pub fn reserve(&mut self, additional: usize, hasher: impl Fn(&T) -> u64) {
        self.raw.reserve(additional, hasher)
    }