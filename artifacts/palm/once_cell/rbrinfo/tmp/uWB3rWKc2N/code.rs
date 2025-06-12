pub fn get(&self) -> Option<NonZeroUsize> {
        let val = self.inner.load(Ordering::Acquire);
        NonZeroUsize::new(val)
    }