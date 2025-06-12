// Answer 0

#[test]
fn test_get_or_try_init_initializes_cell() {
    struct TestOnceCell {
        value: Option<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn set(&mut self, value: i32) -> Result<(), &'static str> {
            if self.value.is_none() {
                self.value = Some(value);
                Ok(())
            } else {
                Err("cell already initialized")
            }
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(self.get().unwrap())
        }
    }

    let mut cell = TestOnceCell::new();
    assert_eq!(cell.get_or_try_init(|| Ok(42)), Ok(&42));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_fails_if_empty_and_fails() {
    struct TestOnceCell {
        value: Option<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn set(&mut self, value: i32) -> Result<(), &'static str> {
            if self.value.is_none() {
                self.value = Some(value);
                Ok(())
            } else {
                Err("cell already initialized")
            }
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(self.get().unwrap())
        }
    }

    let mut cell = TestOnceCell::new();
    assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));
    assert!(cell.get().is_none());
}

#[should_panic]
#[test]
fn test_get_or_try_init_panics_on_reentrant_init() {
    struct TestOnceCell {
        value: Option<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn set(&mut self, value: i32) -> Result<(), &'static str> {
            if self.value.is_none() {
                self.value = Some(value);
                Ok(())
            } else {
                Err("cell already initialized")
            }
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(self.get().unwrap())
        }
    }

    let mut cell = TestOnceCell::new();
    let _ = cell.get_or_try_init(|| {
        cell.get_or_try_init(|| Ok(42)); // Reentrant call
        Ok(0)
    });
}

