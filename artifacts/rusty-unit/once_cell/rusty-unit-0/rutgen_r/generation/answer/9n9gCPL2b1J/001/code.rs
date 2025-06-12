// Answer 0

#[test]
fn test_get_some_value() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct Inner {
        inner: AtomicPtr<i32>,
    }

    impl Inner {
        fn new(value: *mut i32) -> Self {
            Self {
                inner: AtomicPtr::new(value),
            }
        }

        pub fn get(&self) -> Option<&i32> {
            let ptr = self.inner.load(Ordering::Acquire);
            unsafe { ptr.as_ref() }
        }
    }

    let value = Box::new(42);
    let raw_value: *mut i32 = Box::into_raw(value);

    let inner = Inner::new(raw_value);

    assert_eq!(inner.get(), Some(unsafe { &*raw_value }));

    // Clean up
    let _ = unsafe { Box::from_raw(raw_value) };
}

#[test]
fn test_get_none_value() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct Inner {
        inner: AtomicPtr<i32>,
    }

    impl Inner {
        fn new(value: *mut i32) -> Self {
            Self {
                inner: AtomicPtr::new(value),
            }
        }

        pub fn get(&self) -> Option<&i32> {
            let ptr = self.inner.load(Ordering::Acquire);
            unsafe { ptr.as_ref() }
        }
    }

    let inner = Inner::new(ptr::null_mut());

    assert_eq!(inner.get(), None);
}

