// Answer 0

#[test]
fn test_new_creates_empty_cell() {
    struct Cell {
        inner: std::ptr::AtomicPtr<*mut std::ffi::c_void>,
        ghost: std::marker::PhantomData<*const ()>,
    }

    impl Cell {
        pub const fn new() -> Self {
            Self { inner: std::ptr::AtomicPtr::new(std::ptr::null_mut()), ghost: std::marker::PhantomData }
        }
    }

    let cell = Cell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), std::ptr::null_mut());
}

