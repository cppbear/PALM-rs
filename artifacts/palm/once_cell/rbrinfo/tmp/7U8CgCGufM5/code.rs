fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {
        self.inner.compare_exchange(0, val.get(), Ordering::Release, Ordering::Acquire)
    }