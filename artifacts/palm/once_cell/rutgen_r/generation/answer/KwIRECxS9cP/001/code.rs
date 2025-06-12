// Answer 0

#[test]
fn test_get_or_try_init_success_with_existing_value() {
    struct TestCell {
        value: Option<Box<i32>>,
    }

    impl TestCell {
        fn new() -> Self {
            Self { value: None }
        }

        fn get(&self) -> Option<&Box<i32>> {
            self.value.as_ref()
        }

        fn init<F, E>(&mut self, f: F) -> Result<&Box<i32>, E>
        where
            F: FnOnce() -> Result<Box<i32>, E>,
        {
            let result = f()?;
            self.value = Some(result);
            Ok(self.get().unwrap())
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&Box<i32>, E>
        where
            F: FnOnce() -> Result<Box<i32>, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let mut cell = TestCell::new();
    cell.value = Some(Box::new(42));
    
    let result = cell.get_or_try_init(|| Ok(Box::new(100)));
    assert_eq!(*result.unwrap(), 42);
}

#[test]
fn test_get_or_try_init_with_failing_init() {
    struct TestCell {
        value: Option<Box<i32>>,
    }

    impl TestCell {
        fn new() -> Self {
            Self { value: None }
        }

        fn get(&self) -> Option<&Box<i32>> {
            self.value.as_ref()
        }

        fn init<F, E>(&mut self, f: F) -> Result<&Box<i32>, E>
        where
            F: FnOnce() -> Result<Box<i32>, E>,
        {
            let result = f()?;
            self.value = Some(result);
            Ok(self.get().unwrap())
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&Box<i32>, E>
        where
            F: FnOnce() -> Result<Box<i32>, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let mut cell = TestCell::new();
    let result = cell.get_or_try_init(|| Err("Initialization failed"));
    assert!(result.is_err());
}

