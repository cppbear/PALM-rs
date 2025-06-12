// Answer 0

#[test]
fn test_get_or_try_init_with_empty_cell() {
    struct TestCell {
        value: Option<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<&i32, E>,
        {
            let val = f()?;
            self.value = Some(*val);
            Ok(val)
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<&i32, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let mut cell = TestCell::new();

    let init_fn = || Ok(&42);
    let result = cell.get_or_try_init(init_fn);
    
    assert_eq!(result, Ok(&42));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_with_error() {
    struct ErrorCell {
        value: Option<i32>,
    }

    impl ErrorCell {
        fn new() -> Self {
            ErrorCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<&i32, E>,
        {
            f()
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<&i32, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let mut cell = ErrorCell::new();

    let init_fn = || Err("Init failed"); 
    let result = cell.get_or_try_init(init_fn);
    
    assert_eq!(result, Err("Init failed"));
    assert_eq!(cell.get(), None);
}

