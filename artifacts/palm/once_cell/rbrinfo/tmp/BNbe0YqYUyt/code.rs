pub(crate) const fn with_value(value: T) -> OnceCell<T> {
        OnceCell { queue: AtomicPtr::new(COMPLETE_PTR), value: UnsafeCell::new(Some(value)) }
    }