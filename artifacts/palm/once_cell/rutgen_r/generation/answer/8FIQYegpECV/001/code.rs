// Answer 0

#[test]
fn test_set_full_cell() {
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

    // Initialize cell
    let cell: Cell<i32> = Cell::new();
    
    // First set should succeed
    assert!(cell.set(Box::new(1)).is_ok());

    // Second set should fail (exchange.is_err() should be true)
    let result = cell.set(Box::new(2));
    assert!(result.is_err());
    if let Err(err_value) = result {
        assert_eq!(*err_value, 2);
    }
}

