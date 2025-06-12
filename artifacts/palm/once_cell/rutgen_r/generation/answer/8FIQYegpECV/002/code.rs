// Answer 0

#[test]
fn test_set_when_cell_is_empty() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

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
    let result = cell.set(Box::new(42));
    assert!(result.is_ok());
}

#[test]
fn test_set_when_cell_is_full() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

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
    cell.set(Box::new(42)).unwrap(); // Fill the cell first
    let result = cell.set(Box::new(84)); // Attempt to fill it again
    assert!(result.is_err());
}

