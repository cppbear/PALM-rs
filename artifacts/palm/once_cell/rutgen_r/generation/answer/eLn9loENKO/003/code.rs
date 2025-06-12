// Answer 0

#[test]
fn test_init_success_with_exchange() {
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};
    
    struct Inner {
        inner: AtomicPtr<i32>,
    }

    struct TestOnceCell {
        inner: Inner,
    }

    impl TestOnceCell {
        fn init<F, E>(&self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<Box<i32>, E>,
        {
            let val = f()?;
            let mut ptr = Box::into_raw(val);
            let exchange = self.inner.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::Release,
                Ordering::Acquire,
            );
            if let Err(old) = exchange {
                drop(unsafe { Box::from_raw(ptr) });
                ptr = old;
            }
            Ok(unsafe { &*ptr })
        }
    }

    let cell = TestOnceCell {
        inner: Inner {
            inner: AtomicPtr::new(ptr::null_mut()),
        },
    };

    // Simulate a previous initialization to trigger exchange
    let existing_value = Box::new(10);
    cell.inner.inner.store(Box::into_raw(existing_value), Ordering::SeqCst);

    // Perform initialization
    let result = cell.init(|| Ok(Box::new(42)));

    // Check the result
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 10); // should return the existing value
}

#[test]
#[should_panic]
fn test_init_panic_on_failing_function() {
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};
    
    struct Inner {
        inner: AtomicPtr<i32>,
    }

    struct TestOnceCell {
        inner: Inner,
    }

    impl TestOnceCell {
        fn init<F, E>(&self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<Box<i32>, E>,
        {
            let val = f()?;
            let mut ptr = Box::into_raw(val);
            let exchange = self.inner.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::Release,
                Ordering::Acquire,
            );
            if let Err(old) = exchange {
                drop(unsafe { Box::from_raw(ptr) });
                ptr = old;
            }
            Ok(unsafe { &*ptr })
        }
    }

    let cell = TestOnceCell {
        inner: Inner {
            inner: AtomicPtr::new(ptr::null_mut()),
        },
    };

    // Perform initialization with a function that panics
    let _ = cell.init(|| {
        panic!("This function should panic");
    }); // Expects panic
}

