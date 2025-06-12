// Answer 0

#[test]
fn test_set_empty_cell() {
    struct Cell<T> {
        value: Option<&'static T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn compare_exchange(&mut self, value: &'static T) -> Result<(), &'static T> {
            if self.value.is_none() {
                self.value = Some(value);
                Ok(())
            } else {
                Err(value)
            }
        }

        pub fn set(&mut self, value: &'static T) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let mut cell = Cell::new();
    let value = "test value";
    let result = cell.set(&value);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_set_full_cell() {
    struct Cell<T> {
        value: Option<&'static T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn compare_exchange(&mut self, value: &'static T) -> Result<(), &'static T> {
            if self.value.is_none() {
                self.value = Some(value);
                Ok(())
            } else {
                Err(value)
            }
        }

        pub fn set(&mut self, value: &'static T) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let mut cell = Cell::new();
    let value1 = "first value";
    let _ = cell.set(&value1); // Fill the cell first
    let value2 = "second value";
    let result = cell.set(&value2); // Try to fill the cell again
    assert_eq!(result, Err(()));
}

