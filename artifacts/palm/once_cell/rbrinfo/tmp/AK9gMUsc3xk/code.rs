pub const fn new() -> OnceCell<T> {
            OnceCell { inner: UnsafeCell::new(None) }
        }