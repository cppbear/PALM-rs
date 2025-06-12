pub const fn new() -> Self {
        Self { inner: AtomicPtr::new(ptr::null_mut()), ghost: PhantomData }
    }