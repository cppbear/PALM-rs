pub fn get(&self) -> Option<bool> {
        self.inner.get().map(Self::from_usize)
    }