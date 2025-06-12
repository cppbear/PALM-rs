// Answer 0

#[test]
fn test_get_or_init_with_valid_function() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&T, ()>
        where
            F: FnOnce() -> T,
        {
            if let Some(ref value) = self.value {
                Ok(value)
            } else {
                let val = f();
                self.value = Some(val);
                Ok(self.value.as_ref().unwrap())
            }
        }

        fn get_or_init<F>(&mut self, f: F) -> &T
        where
            F: FnOnce() -> T,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<&T, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell: Cell<i32> = Cell::new();
    let result = cell.get_or_init(|| 42);
    assert_eq!(*result, 42);
}

#[test]
#[should_panic]
fn test_get_or_init_with_panic_function() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&T, ()>
        where
            F: FnOnce() -> T,
        {
            if let Some(ref value) = self.value {
                Ok(value)
            } else {
                let val = f();
                self.value = Some(val);
                Ok(self.value.as_ref().unwrap())
            }
        }

        fn get_or_init<F>(&mut self, f: F) -> &T
        where
            F: FnOnce() -> T,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<&T, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell: Cell<i32> = Cell::new();
    let _result = cell.get_or_init(|| panic!("This function panics"));
}

