// Answer 0

#[test]
fn test_with_value() {
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::marker::PhantomData;

    struct Cell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> Cell<T> {
        pub fn with_value(value: Box<T>) -> Self {
            Self { inner: AtomicPtr::new(Box::into_raw(value)), ghost: PhantomData }
        }
    }

    let boxed_value = Box::new(42);
    let cell: Cell<i32> = Cell::with_value(boxed_value);

    assert!(!cell.inner.load(Ordering::SeqCst).is_null());
    unsafe {
        assert_eq!(*cell.inner.load(Ordering::SeqCst), 42);
    }
}

#[test]
fn test_with_value_null() {
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::marker::PhantomData;

    struct Cell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> Cell<T> {
        pub fn with_value(value: Box<T>) -> Self {
            Self { inner: AtomicPtr::new(Box::into_raw(value)), ghost: PhantomData }
        }
    }

    let cell: Cell<i32> = Cell::with_value(Box::new(0));
    let ptr = cell.inner.load(Ordering::SeqCst);
    unsafe {
        Box::from_raw(ptr); // Prevent memory leak
    }
    assert!(cell.inner.load(Ordering::SeqCst).is_null());
}

