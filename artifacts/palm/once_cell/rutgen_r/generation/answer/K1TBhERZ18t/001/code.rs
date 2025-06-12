// Answer 0

#[test]
fn test_with_value() {
    use std::ptr::null_mut;
    use std::sync::atomic::AtomicPtr;
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

    // Test with a valid value
    let value = Box::new(42);
    let cell = Cell::with_value(value);

    // Ensure that the inner atomic pointer is not null and points to the correct value
    assert!(!cell.inner.load(std::sync::atomic::Ordering::SeqCst).is_null());
    unsafe {
        assert_eq!(*cell.inner.load(std::sync::atomic::Ordering::SeqCst), 42);
    }

    // Test with a different value
    let value_str = Box::new(String::from("Hello"));
    let cell_str = Cell::with_value(value_str);

    assert!(!cell_str.inner.load(std::sync::atomic::Ordering::SeqCst).is_null());
    unsafe {
        assert_eq!(*cell_str.inner.load(std::sync::atomic::Ordering::SeqCst), "Hello");
    }
}

#[should_panic]
#[test]
fn test_with_value_null() {
    struct Cell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> Cell<T> {
        pub fn with_value(value: Box<T>) -> Self {
            Self { inner: AtomicPtr::new(Box::into_raw(value)), ghost: PhantomData }
        }
    }

    // Attempt to create a Cell with a null box, this should panic
    let null_value: Box<i32> = unsafe { Box::from_raw(null_mut()) };
    let _ = Cell::with_value(null_value);
}

