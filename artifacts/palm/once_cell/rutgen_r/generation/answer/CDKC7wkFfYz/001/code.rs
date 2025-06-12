// Answer 0

#[test]
fn test_new_empty_cell() {
    use std::sync::atomic::{AtomicPtr};
    use std::marker::PhantomData;

    struct EmptyCell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> EmptyCell<T> {
        pub const fn new() -> Self {
            Self { inner: AtomicPtr::new(std::ptr::null_mut()), ghost: PhantomData }
        }
    }

    let cell: EmptyCell<i32> = EmptyCell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), std::ptr::null_mut());
}

