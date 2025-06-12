// Answer 0

#[test]
fn test_get_or_init_initializes_empty_cell() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&T, ()>
        where
            F: FnOnce() -> Result<&T, ()>,
        {
            if self.value.is_none() {
                self.value = Some(f()?);
            }
            Ok(self.value.as_ref().unwrap())
        }
        
        fn get_or_init<F>(&mut self, f: F) -> &T
        where
            F: FnOnce() -> &T,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell = Cell::new();
    let val = cell.get_or_init(|| &42);
    assert_eq!(*val, 42);
}

#[test]
fn test_get_or_init_reuses_value() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&T, ()>
        where
            F: FnOnce() -> Result<&T, ()>,
        {
            if self.value.is_none() {
                self.value = Some(f()?);
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &T
        where
            F: FnOnce() -> &T,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell = Cell::new();
    let first_val = cell.get_or_init(|| &42);
    let second_val = cell.get_or_init(|| &100);
    assert_eq!(*first_val, 42);
    assert_eq!(first_val, second_val);
}

