pub const fn new() -> Self {
        Self { inner: OnceNonZeroUsize::new() }
    }