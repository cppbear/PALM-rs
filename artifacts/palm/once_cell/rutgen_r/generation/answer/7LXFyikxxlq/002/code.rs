// Answer 0

#[test]
fn test_get_or_try_init_empty_cell_success() {
    struct TestCell<'a> {
        value: Option<&'a str>,
    }

    impl<'a> TestCell<'a> {
        fn new() -> Self {
            TestCell { value: None }
        }

        fn get(&self) -> Option<&'a str> {
            self.value
        }

        fn init<F, E>(&mut self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            match f() {
                Ok(val) => {
                    self.value = Some(val);
                    Ok(val)
                }
                Err(err) => Err(err),
            }
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let mut cell = TestCell::new();
    let result = cell.get_or_try_init(|| Ok("Initialized Value")).unwrap();
    assert_eq!(result, "Initialized Value");
}

#[test]
fn test_get_or_try_init_empty_cell_failure() {
    struct TestCell<'a> {
        value: Option<&'a str>,
    }

    impl<'a> TestCell<'a> {
        fn new() -> Self {
            TestCell { value: None }
        }

        fn get(&self) -> Option<&'a str> {
            self.value
        }

        fn init<F, E>(&mut self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            match f() {
                Ok(val) => {
                    self.value = Some(val);
                    Ok(val)
                }
                Err(err) => Err(err),
            }
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let mut cell = TestCell::new();
    let result: Result<&str, &str> = cell.get_or_try_init(|| Err("Initialization Failed"));
    assert_eq!(result, Err("Initialization Failed"));
}

