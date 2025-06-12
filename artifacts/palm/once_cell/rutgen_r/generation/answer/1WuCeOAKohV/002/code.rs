// Answer 0

#[test]
fn test_get_or_init_with_successful_initialization() {
    struct CellWrapper {
        value: Option<Box<i32>>,
    }

    impl CellWrapper {
        fn new() -> Self {
            CellWrapper { value: None }
        }

        fn get_or_try_init<F>(&mut self, init: F) -> Result<&Box<i32>, ()>
        where
            F: FnOnce() -> Result<Box<i32>, ()>,
        {
            if self.value.is_none() {
                self.value = Some(init().map_err(|_| ())?);
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &Box<i32>
        where
            F: FnOnce() -> Box<i32>,
        {
            match self.get_or_try_init(|| Ok::<Box<i32>, ()>(f())) {
                Ok(val) => val,
                Err(_) => panic!("Initialization failed"),
            }
        }
    }

    let mut cell = CellWrapper::new();
    let value = cell.get_or_init(|| Box::new(42));
    assert_eq!(*value, 42);
}

#[test]
#[should_panic]
fn test_get_or_init_with_initialization_panicking() {
    struct CellWrapper {
        value: Option<Box<i32>>,
    }

    impl CellWrapper {
        fn new() -> Self {
            CellWrapper { value: None }
        }

        fn get_or_try_init<F>(&mut self, init: F) -> Result<&Box<i32>, ()>
        where
            F: FnOnce() -> Result<Box<i32>, ()>,
        {
            if self.value.is_none() {
                self.value = Some(init().map_err(|_| ())?);
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &Box<i32>
        where
            F: FnOnce() -> Box<i32>,
        {
            match self.get_or_try_init(|| Ok::<Box<i32>, ()>(f())) {
                Ok(val) => val,
                Err(_) => panic!("Initialization failed"),
            }
        }
    }

    let mut cell = CellWrapper::new();
    let _value = cell.get_or_init(|| panic!("Forced panic for testing"));
}

