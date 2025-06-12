// Answer 0

#[test]
fn test_get_with_value() {
    use std::cell::UnsafeCell;

    struct OnceCell<T> {
        inner: UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        pub fn new() -> Self {
            OnceCell {
                inner: UnsafeCell::new(None),
            }
        }

        pub fn set(&self, value: T) {
            unsafe {
                *self.inner.get() = Some(value);
            }
        }

        pub fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }
    }

    let cell = OnceCell::new();
    cell.set(42);
    let value = cell.get();
    assert_eq!(value, Some(&42));
}

#[test]
fn test_get_empty_cell() {
    use std::cell::UnsafeCell;

    struct OnceCell<T> {
        inner: UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        pub fn new() -> Self {
            OnceCell {
                inner: UnsafeCell::new(None),
            }
        }

        pub fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }
    }

    let cell = OnceCell::new();
    let value = cell.get();
    assert_eq!(value, None);
}

#[should_panic]
#[test]
fn test_get_after_multiple_set() {
    use std::cell::UnsafeCell;

    struct OnceCell<T> {
        inner: UnsafeCell<Option<T>>,
    }

    impl<T> OnceCell<T> {
        pub fn new() -> Self {
            OnceCell {
                inner: UnsafeCell::new(None),
            }
        }

        pub fn set(&self, value: T) {
            unsafe {
                *self.inner.get() = Some(value);
            }
        }

        pub fn get(&self) -> Option<&T> {
            unsafe { &*self.inner.get() }.as_ref()
        }
    }

    let cell = OnceCell::new();
    cell.set(42);
    cell.set(100); // This should trigger a panic in a real implementation as it violates the constraints.
    let value = cell.get();
    assert_eq!(value, Some(&100));
}

