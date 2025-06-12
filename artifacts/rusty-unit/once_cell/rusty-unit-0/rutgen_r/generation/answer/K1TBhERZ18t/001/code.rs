// Answer 0

#[test]
fn test_with_value_valid() {
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

    let value = Box::new(42);
    let cell = Cell::with_value(value);

    assert!(!cell.inner.load(Ordering::SeqCst).is_null());
}

#[test]
#[should_panic]
fn test_with_value_empty_box() {
    use std::marker::PhantomData;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Cell<T> {
        inner: AtomicPtr<T>,
        ghost: PhantomData<T>,
    }

    impl<T> Cell<T> {
        pub fn with_value(value: Box<T>) -> Self {
            Self { inner: AtomicPtr::new(Box::into_raw(value)), ghost: PhantomData }
        }
    }

    // Attempt to create a Cell with a null pointer (simulating behavior of an empty Box).
    let value: Box<i32> = Box::new(0);
    let cell = Cell::with_value(value);
    let ptr = cell.inner.load(Ordering::SeqCst);
    unsafe {
        let _ = Box::from_raw(ptr); // this should not panic
    }
}

