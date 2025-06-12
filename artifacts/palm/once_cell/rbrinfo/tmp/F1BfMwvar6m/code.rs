pub(crate) const fn new() -> OnceCell<T> {
        OnceCell { queue: AtomicPtr::new(INCOMPLETE_PTR), value: UnsafeCell::new(None) }
    }