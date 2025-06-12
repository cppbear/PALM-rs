// Answer 0

#[test]
fn test_new_empty_cell() {
    use std::ptr::null_mut;
    use std::sync::atomic::AtomicPtr;
    use std::marker::PhantomData;

    struct EmptyCell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> EmptyCell<T> {
        pub const fn new() -> Self {
            Self { inner: AtomicPtr::new(null_mut()), ghost: PhantomData }
        }
    }

    let cell: EmptyCell<()> = EmptyCell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), null_mut());
}

