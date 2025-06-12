// Answer 0

#[test]
fn test_init_success() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct TestStruct {
        inner: AtomicPtr<i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                inner: AtomicPtr::new(std::ptr::null_mut()),
            }
        }

        fn init<E>(&self, f: impl FnOnce() -> Result<Box<i32>, E>) -> Result<&i32, E> {
            let val = f()?;
            let mut ptr = Box::into_raw(val);
            let exchange = self.inner.compare_exchange(
                std::ptr::null_mut(),
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

    let test_struct = TestStruct::new();
    let result = test_struct.init(|| Ok(Box::new(42)));
    assert_eq!(*result.unwrap(), 42);
}

#[test]
#[should_panic]
fn test_init_double_init() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct TestStruct {
        inner: AtomicPtr<i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                inner: AtomicPtr::new(std::ptr::null_mut()),
            }
        }

        fn init<E>(&self, f: impl FnOnce() -> Result<Box<i32>, E>) -> Result<&i32, E> {
            let val = f()?;
            let mut ptr = Box::into_raw(val);
            let exchange = self.inner.compare_exchange(
                std::ptr::null_mut(),
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

    let test_struct = TestStruct::new();
    let _ = test_struct.init(|| Ok(Box::new(42)));
    // Trying to initialize again should panic due to the expected panic behavior
    let _ = test_struct.init(|| Ok(Box::new(43)));
}

