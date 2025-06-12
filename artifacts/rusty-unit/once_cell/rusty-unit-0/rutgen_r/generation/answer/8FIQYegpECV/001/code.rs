// Answer 0

#[derive(Debug)]
struct MockCell<T> {
    inner: std::sync::atomic::AtomicPtr<T>,
}

impl<T> MockCell<T> {
    pub fn new() -> Self {
        Self {
            inner: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    pub fn set(&self, value: Box<T>) -> Result<(), Box<T>> {
        let ptr = Box::into_raw(value);
        let exchange = self.inner.compare_exchange(
            std::ptr::null_mut(),
            ptr,
            std::sync::atomic::Ordering::Release,
            std::sync::atomic::Ordering::Acquire,
        );
        if exchange.is_err() {
            let value = unsafe { Box::from_raw(ptr) };
            return Err(value);
        }
        Ok(())
    }
}

#[test]
fn test_set_when_full() {
    let cell = MockCell::new();
    
    // Initially set the cell to a value to make it full
    let _ = cell.set(Box::new(42)); // First call, should succeed

    // Now attempt to set another value, which should fail
    let result = cell.set(Box::new(99)); // Second call, should fail
    assert!(result.is_err()); // We expect an Err here

    // Furthermore, test that the returned value is what we expected
    if let Err(value) = result {
        assert_eq!(*value, 99); // The returned value should be the one attempted to set
    } else {
        panic!("Expected an error from the set function");
    }
}

#[test]
#[should_panic]
fn test_set_with_null() {
    let cell = MockCell::new();
    
    // Attempt to set a null value should panic
    let _ = cell.set(Box::from_raw(std::ptr::null_mut()));
}

