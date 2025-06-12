pub const fn new() -> Self {
        Self { inner: AtomicUsize::new(0) }
    }