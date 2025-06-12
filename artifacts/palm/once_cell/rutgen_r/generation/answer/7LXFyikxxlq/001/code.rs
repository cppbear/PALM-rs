// Answer 0

#[test]
fn test_get_or_try_init_with_some_value() {
    struct Cell<'a>(&'a str);
    
    impl<'a> Cell<'a> {
        fn get(&self) -> Option<&'a str> {
            Some(self.0)
        }
        
        fn init<F, E>(&self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            f()
        }
        
        fn get_or_try_init<F, E>(&self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let cell = Cell("Test value");
    let result: Result<&str, &str> = cell.get_or_try_init(|| Ok("Initialized value"));

    assert_eq!(result, Ok("Test value"));
}

#[test]
fn test_get_or_try_init_with_some_value_multiple_calls() {
    struct Cell<'a>(&'a str);

    impl<'a> Cell<'a> {
        fn get(&self) -> Option<&'a str> {
            Some(self.0)
        }
        
        fn init<F, E>(&self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            f()
        }
        
        fn get_or_try_init<F, E>(&self, f: F) -> Result<&'a str, E>
        where
            F: FnOnce() -> Result<&'a str, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let cell = Cell("Concurrent Test value");
    let result1: Result<&str, &str> = cell.get_or_try_init(|| Ok("Initialized value 1"));
    let result2: Result<&str, &str> = cell.get_or_try_init(|| Ok("Initialized value 2"));

    assert_eq!(result1, Ok("Concurrent Test value"));
    assert_eq!(result2, Ok("Concurrent Test value"));
}

