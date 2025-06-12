// Answer 0

#[test]
fn test_get_or_try_init_with_existing_value() {
    struct TestCell {
        inner: UnsafeCell<Option<i32>>,
    }

    impl TestCell {
        const fn new() -> Self {
            Self {
                inner: UnsafeCell::new(Some(42)),
            }
        }

        fn get(&self) -> Option<&i32> {
            unsafe { &*self.inner.get() }.as_ref()
        }

        fn set(&self, value: i32) -> Result<(), i32> {
            if unsafe { &*self.inner.get() }.is_none() {
                unsafe { *self.inner.get() = Some(value) };
                Ok(())
            } else {
                Err(value)
            }
        }

        fn get_or_try_init<F, E>(&self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(unsafe { self.get().unwrap() })
        }
    }

    let cell = TestCell::new();
    assert_eq!(cell.get_or_try_init(|| Ok(99)), Ok(&42));
}

#[test]
#[should_panic(expected = "reentrant init")]
fn test_get_or_try_init_reentrant_init_panics() {
    struct TestCell {
        inner: UnsafeCell<Option<i32>>,
    }

    impl TestCell {
        const fn new() -> Self {
            Self {
                inner: UnsafeCell::new(None),
            }
        }

        fn get(&self) -> Option<&i32> {
            unsafe { &*self.inner.get() }.as_ref()
        }

        fn set(&self, value: i32) -> Result<(), i32> {
            if unsafe { &*self.inner.get() }.is_none() {
                unsafe { *self.inner.get() = Some(value) };
                Ok(())
            } else {
                Err(value)
            }
        }

        fn get_or_try_init<F, E>(&self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(unsafe { self.get().unwrap() })
        }
    }

    let cell = TestCell::new();
    let _ = cell.get_or_try_init(|| {
        cell.get_or_try_init(|| Ok(100)); // Reentrant call
        Ok(42)
    });
}

#[test]
fn test_get_or_try_init_fails_initialization() {
    struct TestCell {
        inner: UnsafeCell<Option<i32>>,
    }

    impl TestCell {
        const fn new() -> Self {
            Self {
                inner: UnsafeCell::new(None),
            }
        }

        fn get(&self) -> Option<&i32> {
            unsafe { &*self.inner.get() }.as_ref()
        }

        fn set(&self, value: i32) -> Result<(), i32> {
            if unsafe { &*self.inner.get() }.is_none() {
                unsafe { *self.inner.get() = Some(value) };
                Ok(())
            } else {
                Err(value)
            }
        }

        fn get_or_try_init<F, E>(&self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(unsafe { self.get().unwrap() })
        }
    }

    let cell = TestCell::new();
    assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));
}

