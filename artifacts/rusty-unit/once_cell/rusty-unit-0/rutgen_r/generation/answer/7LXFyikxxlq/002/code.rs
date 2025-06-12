// Answer 0

#[test]
fn test_get_or_try_init_empty_cell_success() {
    struct TestCell<T> {
        value: Option<T>,
    }

    impl<T> TestCell<T> {
        fn new() -> Self {
            TestCell { value: None }
        }

        fn get(&self) -> Option<&T> {
            self.value.as_ref()
        }

        fn init<F, E>(&mut self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {
            let result = f()?;
            self.value = Some(result);
            self.get().ok_or_else(|| panic!("Expected a value after init"))
        }
    }

    let mut cell: TestCell<i32> = TestCell::new();
    let result: Result<&i32, &str> = cell.get_or_try_init(|| Ok(&42));
    assert_eq!(result.unwrap(), &42);
}

#[test]
fn test_get_or_try_init_empty_cell_failure() {
    struct TestCell<T> {
        value: Option<T>,
    }

    impl<T> TestCell<T> {
        fn new() -> Self {
            TestCell { value: None }
        }

        fn get(&self) -> Option<&T> {
            self.value.as_ref()
        }

        fn init<F, E>(&mut self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {
            let result = f()?;
            self.value = Some(result);
            self.get().ok_or_else(|| panic!("Expected a value after init"))
        }
    }

    let mut cell: TestCell<i32> = TestCell::new();
    let result: Result<&i32, &str> = cell.get_or_try_init(|| Err("Initialization failed"));
    assert!(result.is_err());
}

