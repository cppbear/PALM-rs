// Answer 0

#[test]
fn test_compare_exchange_success() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct Inner {
        inner: AtomicPtr<i32>,
    }

    impl Inner {
        fn compare_exchange(&self, old: *mut i32, new: *mut i32, success_order: Ordering, failure_order: Ordering) -> Result<*mut i32, *mut i32> {
            self.inner.compare_exchange(old, new, success_order, failure_order)
        }
    }

    let value = Box::new(10);
    let inner = Inner {
        inner: AtomicPtr::new(ptr::null_mut()),
    };

    let result = inner.compare_exchange(ptr::null_mut(), Box::into_raw(value), Ordering::Release, Ordering::Acquire);
    assert!(result.is_ok());
}

#[test]
fn test_compare_exchange_failure() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct Inner {
        inner: AtomicPtr<i32>,
    }

    impl Inner {
        fn compare_exchange(&self, old: *mut i32, new: *mut i32, success_order: Ordering, failure_order: Ordering) -> Result<*mut i32, *mut i32> {
            self.inner.compare_exchange(old, new, success_order, failure_order)
        }
    }

    let value1 = Box::new(10);
    let value2 = Box::new(20);
    let inner = Inner {
        inner: AtomicPtr::new(Box::into_raw(value1)),
    };

    let result = inner.compare_exchange(Box::into_raw(value2), Box::into_raw(value2), Ordering::Release, Ordering::Acquire);
    assert!(result.is_err());
}

