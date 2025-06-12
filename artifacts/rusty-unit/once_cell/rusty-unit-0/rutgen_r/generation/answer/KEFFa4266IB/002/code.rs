// Answer 0

#[test]
fn test_set_success() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn compare_exchange(&mut self, new_value: &T) -> Result<&T, ()> {
            if self.value.is_none() {
                self.value = Some(new_value.clone());
                Ok(new_value)
            } else {
                Err(())
            }
        }

        pub fn set(&mut self, value: T) -> Result<(), ()> {
            match self.compare_exchange(&value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let mut cell = Cell::new();
    let result = cell.set(42);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_set_failure() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T: Clone> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn compare_exchange(&mut self, new_value: &T) -> Result<&T, ()> {
            if self.value.is_none() {
                self.value = Some(new_value.clone());
                Ok(new_value)
            } else {
                Err(())
            }
        }

        pub fn set(&mut self, value: T) -> Result<(), ()> {
            match self.compare_exchange(&value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let mut cell = Cell::new();
    let _ = cell.set(42);
    let result = cell.set(99);
    assert_eq!(result, Err(()));
}

