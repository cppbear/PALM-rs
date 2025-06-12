pub fn shrink_to_fit(&mut self, hasher: impl Fn(&T) -> u64) {
        self.raw.shrink_to(self.len(), hasher)
    }