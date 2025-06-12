// Answer 0

#[test]
fn test_get_with_value() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct OnceCell<T> {
        inner: AtomicPtr<T>,
    }

    impl<T> OnceCell<T> {
        pub fn new() -> Self {
            OnceCell {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        pub fn set(&self, value: T) {
            let boxed = Box::new(value);
            let ptr = Box::into_raw(boxed);
            self.inner.store(ptr, Ordering::Release);
        }

        pub fn get(&self) -> Option<&T> {
            let ptr = self.inner.load(Ordering::Acquire);
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }
    }

    let cell = OnceCell::new();
    cell.set(42);
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_get_without_value() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct OnceCell<T> {
        inner: AtomicPtr<T>,
    }

    impl<T> OnceCell<T> {
        pub fn new() -> Self {
            OnceCell {
                inner: AtomicPtr::new(ptr::null_mut()),
            }
        }

        pub fn get(&self) -> Option<&T> {
            let ptr = self.inner.load(Ordering::Acquire);
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }
    }

    let cell = OnceCell::new();
    assert_eq!(cell.get(), None);
}

