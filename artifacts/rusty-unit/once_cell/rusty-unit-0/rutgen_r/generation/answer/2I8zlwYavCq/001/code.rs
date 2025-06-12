// Answer 0

#[test]
fn test_compare_exchange_success() {
    use std::sync::{Arc, Mutex};
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Inner {
        inner: AtomicPtr<i32>,
    }

    impl Inner {
        fn new() -> Self {
            Inner {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        fn compare_exchange(&self, value: *const i32) -> Result<(), *const i32> {
            self.inner
                .compare_exchange(
                    ptr::null_mut(),
                    value as *mut i32,
                    Ordering::Release,
                    Ordering::Acquire,
                )
                .map(|_: *mut i32| ())
                .map_err(|err| err as *const i32)
        }
    }

    let inner = Inner::new();
    let value: i32 = 42;
    let value_ptr: *const i32 = &value;

    assert!(inner.compare_exchange(value_ptr).is_ok());
}

#[test]
fn test_compare_exchange_fail() {
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Inner {
        inner: AtomicPtr<i32>,
    }

    impl Inner {
        fn new() -> Self {
            Inner {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        fn compare_exchange(&self, value: *const i32) -> Result<(), *const i32> {
            self.inner
                .compare_exchange(
                    ptr::null_mut(),
                    value as *mut i32,
                    Ordering::Release,
                    Ordering::Acquire,
                )
                .map(|_: *mut i32| ())
                .map_err(|err| err as *const i32)
        }
    }

    let inner = Inner::new();
    let value: i32 = 42;
    let value_ptr: *const i32 = &value;

    inner.compare_exchange(value_ptr).unwrap();

    assert_eq!(inner.compare_exchange(value_ptr).is_err(), true);
} 

#[should_panic]
#[test]
fn test_compare_exchange_panic() {
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Inner {
        inner: AtomicPtr<i32>,
    }

    impl Inner {
        fn new() -> Self {
            Inner {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        fn compare_exchange(&self, value: *const i32) -> Result<(), *const i32> {
            self.inner
                .compare_exchange(
                    ptr::null_mut(),
                    value as *mut i32,
                    Ordering::Release,
                    Ordering::Acquire,
                )
                .map(|_: *mut i32| ())
                .map_err(|err| err as *const i32)
        }
    }

    let inner = Inner::new();

    // This should cause a panic because we're passing a dangling pointer
    let value_ptr: *const i32 = ptr::null();
    inner.compare_exchange(value_ptr).unwrap();
}

