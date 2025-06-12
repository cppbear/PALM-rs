// Answer 0

#[test]
fn test_try_insert_already_initialized() {
    struct TestOnceCell<T> {
        inner: std::cell::RefCell<Option<T>>,
    }

    impl<T> TestOnceCell<T> {
        fn new() -> Self {
            TestOnceCell {
                inner: std::cell::RefCell::new(None),
            }
        }

        fn get(&self) -> Option<&T> {
            self.inner.borrow().as_ref()
        }

        fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
            if let Some(old) = self.get() {
                return Err((old, value));
            }

            let slot = self.inner.borrow_mut();
            *slot = Some(value);
            Ok(slot.as_ref().unwrap())
        }
    }

    let cell = TestOnceCell::new();
    assert!(cell.get().is_none());

    // First insertion, should succeed
    assert_eq!(cell.try_insert(92), Ok(&92));

    // Attempt to insert another value, should return Err
    assert_eq!(cell.try_insert(62), Err((&92, 62)));
}

#[test]
#[should_panic]
fn test_try_insert_with_unchecked_panic() {
    struct TestOnceCell<T> {
        inner: std::cell::RefCell<Option<T>>,
    }

    impl<T> TestOnceCell<T> {
        fn new() -> Self {
            TestOnceCell {
                inner: std::cell::RefCell::new(None),
            }
        }

        fn get(&self) -> Option<&T> {
            self.inner.borrow().as_ref()
        }

        fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
            if let Some(old) = self.get() {
                return Err((old, value));
            }

            let mut slot = self.inner.borrow_mut();
            *slot = Some(value);
            // This will panic because we just inserted the value, so it's not accessible via unsafe
            Ok(unsafe { slot.as_ref().unwrap_unchecked() })
        }
    }

    let cell = TestOnceCell::new();
    // Insert to be able to trigger the panic on subsequent incorrect unsafe access
    assert_eq!(cell.try_insert(92), Ok(&92));
    // Insert once more without checking, should panic
    let _ = cell.try_insert(62);
}

