// Answer 0

#[test]
fn test_set_empty_cell() {
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Cell<T> {
        inner: AtomicPtr<T>,
    }

    impl<T> Cell<T> {
        pub fn new() -> Self {
            Cell {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        pub fn set(&self, value: Box<T>) -> Result<(), Box<T>> {
            let ptr = Box::into_raw(value);
            let exchange = self.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::Release,
                Ordering::Acquire,
            );
            if exchange.is_err() {
                let value = unsafe { Box::from_raw(ptr) };
                return Err(value);
            }
            Ok(())
        }
    }

    let cell: Cell<i32> = Cell::new();
    assert!(cell.set(Box::new(42)).is_ok());
}

#[test]
fn test_set_filled_cell() {
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Cell<T> {
        inner: AtomicPtr<T>,
    }

    impl<T> Cell<T> {
        pub fn new() -> Self {
            Cell {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        pub fn set(&self, value: Box<T>) -> Result<(), Box<T>> {
            let ptr = Box::into_raw(value);
            let exchange = self.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::Release,
                Ordering::Acquire,
            );
            if exchange.is_err() {
                let value = unsafe { Box::from_raw(ptr) };
                return Err(value);
            }
            Ok(())
        }
    }

    let cell: Cell<i32> = Cell::new();
    assert!(cell.set(Box::new(42)).is_ok());
    let result = cell.set(Box::new(43));
    assert!(result.is_err());
    assert_eq!(*result.err().unwrap(), 43);
}

