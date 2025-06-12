pub const fn with_value(value: T) -> OnceCell<T> {
            OnceCell { inner: UnsafeCell::new(Some(value)) }
        }