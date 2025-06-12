// Answer 0

#[test]
fn test_new_empty_cell() {
    use std::sync::atomic::AtomicPtr;
    use std::marker::PhantomData;
    use std::ptr;

    struct Cell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> Cell<T> {
        pub const fn new() -> Self {
            Self { inner: AtomicPtr::new(ptr::null_mut()), ghost: PhantomData }
        }
    }

    // Create a new empty cell
    let cell: Cell<i32> = Cell::new();

    // Check the inner pointer is null
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), ptr::null_mut());
}

