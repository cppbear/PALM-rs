// Answer 0

#[test]
fn test_new() {
    use std::ptr;
    use std::sync::atomic::AtomicPtr;
    use std::marker::PhantomData;

    struct Cell {
        inner: AtomicPtr<u8>,
        ghost: PhantomData<u8>,
    }

    impl Cell {
        pub const fn new() -> Self {
            Self { inner: AtomicPtr::new(ptr::null_mut()), ghost: PhantomData }
        }
    }

    let cell = Cell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), ptr::null_mut());
}

