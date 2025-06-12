// Answer 0

#[test]
fn test_new_cell() {
    use std::ptr;
    use std::sync::atomic::AtomicPtr;
    use std::marker::PhantomData;

    struct Cell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> Cell<T> {
        pub const fn new() -> Self {
            Self {
                inner: AtomicPtr::new(ptr::null_mut()),
                ghost: PhantomData,
            }
        }
    }

    let cell: Cell<i32> = Cell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), ptr::null_mut());
}

