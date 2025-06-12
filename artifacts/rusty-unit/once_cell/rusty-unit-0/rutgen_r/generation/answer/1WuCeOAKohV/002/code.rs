// Answer 0

#[test]
fn test_get_or_init_with_valid_function() {
    struct TestCell {
        initialized: bool,
        value: Option<Box<i32>>,
    }

    impl TestCell {
        pub fn new() -> Self {
            Self {
                initialized: false,
                value: None,
            }
        }

        pub fn get_or_try_init<F>(&mut self, f: F) -> Result<&Box<i32>, &'static str>
        where
            F: FnOnce() -> Result<Box<i32>, ()>,
        {
            if self.initialized {
                Ok(self.value.as_ref().unwrap())
            } else {
                let result = f();
                match result {
                    Ok(val) => {
                        self.initialized = true;
                        self.value = Some(val);
                        Ok(self.value.as_ref().unwrap())
                    },
                    Err(_) => Err("Initialization failed"),
                }
            }
        }

        pub fn get_or_init<F>(&mut self, f: F) -> &Box<i32>
        where
            F: FnOnce() -> Box<i32>,
        {
            match self.get_or_try_init(|| Ok::<Box<i32>, ()>(f())) {
                Ok(val) => val,
                Err(_) => panic!("Initialization should not fail"),
            }
        }
    }

    let mut cell = TestCell::new();
    let value = cell.get_or_init(|| Box::new(42));
    assert_eq!(*value, 42);
}

#[test]
fn test_get_or_init_multiple_initializations() {
    struct TestCell {
        initialized: bool,
        value: Option<Box<i32>>,
    }

    impl TestCell {
        pub fn new() -> Self {
            Self {
                initialized: false,
                value: None,
            }
        }

        pub fn get_or_try_init<F>(&mut self, f: F) -> Result<&Box<i32>, &'static str>
        where
            F: FnOnce() -> Result<Box<i32>, ()>,
        {
            if self.initialized {
                Ok(self.value.as_ref().unwrap())
            } else {
                let result = f();
                match result {
                    Ok(val) => {
                        self.initialized = true;
                        self.value = Some(val);
                        Ok(self.value.as_ref().unwrap())
                    },
                    Err(_) => Err("Initialization failed"),
                }
            }
        }

        pub fn get_or_init<F>(&mut self, f: F) -> &Box<i32>
        where
            F: FnOnce() -> Box<i32>,
        {
            match self.get_or_try_init(|| Ok::<Box<i32>, ()>(f())) {
                Ok(val) => val,
                Err(_) => panic!("Initialization should not fail"),
            }
        }
    }

    let mut cell = TestCell::new();
    let value1 = cell.get_or_init(|| Box::new(100));
    let value2 = cell.get_or_init(|| Box::new(200)); // This should not call f again
    assert_eq!(*value1, 100);
    assert_eq!(*value2, 100);
}

