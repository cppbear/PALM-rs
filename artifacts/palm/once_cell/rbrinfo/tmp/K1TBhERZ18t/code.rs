pub fn with_value(value: Box<T>) -> Self {
            Self { inner: AtomicPtr::new(Box::into_raw(value)), ghost: PhantomData }
        }