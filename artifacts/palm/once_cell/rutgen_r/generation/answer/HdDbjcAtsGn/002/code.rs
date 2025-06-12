// Answer 0

#[test]
fn test_get_or_init_initializes_correctly() {
    struct TestCell {
        value: Option<i32>,
    }
    
    impl TestCell {
        fn new() -> Self {
            TestCell { value: None }
        }
        
        fn get_or_try_init<F>(&mut self, f: F) -> Result<&i32, ()>
        where
            F: FnOnce() -> i32,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &i32
        where
            F: FnOnce() -> i32,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<i32, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell = TestCell::new();
    let value = cell.get_or_init(|| 42);
    assert_eq!(*value, 42);
}

#[test]
#[should_panic]
fn test_get_or_init_panics_on_reentrant_init() {
    struct TestCell {
        value: Option<i32>,
    }
    
    impl TestCell {
        fn new() -> Self {
            TestCell { value: None }
        }
        
        fn get_or_try_init<F>(&mut self, f: F) -> Result<&i32, ()>
        where
            F: FnOnce() -> i32,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &i32
        where
            F: FnOnce() -> i32,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<i32, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell = TestCell::new();
    let _ = cell.get_or_init(|| {
        cell.get_or_init(|| 50); // This will create a reentrant call.
        0
    });
}

#[test]
fn test_get_or_init_no_reinit_on_success() {
    struct TestCell {
        value: Option<String>,
    }
    
    impl TestCell {
        fn new() -> Self {
            TestCell { value: None }
        }
        
        fn get_or_try_init<F>(&mut self, f: F) -> Result<&String, ()>
        where
            F: FnOnce() -> String,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &String
        where
            F: FnOnce() -> String,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<String, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell = TestCell::new();
    let value1 = cell.get_or_init(|| "Hello".to_string());
    assert_eq!(*value1, "Hello");
    let value2 = cell.get_or_init(|| "World".to_string());
    assert_eq!(*value2, "Hello"); // should not initialize again
}

