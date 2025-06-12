pub fn get_or_init<F>(&self, f: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        Self::from_usize(self.inner.get_or_init(|| Self::to_usize(f())))
    }