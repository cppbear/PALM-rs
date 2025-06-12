// Answer 0

#[test]
fn test_get_or_init_with_valid_function() {
    struct Cell<'a, T> {
        value: Option<&'a T>,
    }

    impl<'a, T> Cell<'a, T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&'a T, !>
        where
            F: FnOnce() -> &'a T,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &'a T
        where
            F: FnOnce() -> &'a T,
        {
            match self.get_or_try_init(|| Ok::<&'a T, !>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell = Cell::new();
    let result = cell.get_or_init(|| &42);
    assert_eq!(*result, 42);
}

#[test]
fn test_get_or_init_reuses_value() {
    struct Cell<'a, T> {
        value: Option<&'a T>,
    }

    impl<'a, T> Cell<'a, T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&'a T, !>
        where
            F: FnOnce() -> &'a T,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            Ok(self.value.as_ref().unwrap())
        }

        fn get_or_init<F>(&mut self, f: F) -> &'a T
        where
            F: FnOnce() -> &'a T,
        {
            match self.get_or_try_init(|| Ok::<&'a T, !>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell = Cell::new();
    let first_result = cell.get_or_init(|| &42);
    let second_result = cell.get_or_init(|| &100);
    assert_eq!(first_result, second_result);
    assert_eq!(*first_result, 42);
}

