// Answer 0

#[test]
fn test_set_when_cell_is_full() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T: PartialEq> Cell<T> {
        pub fn new() -> Self {
            Cell { value: None }
        }

        pub fn compare_exchange(&self, value: &T) -> Result<(), ()> {
            match &self.value {
                Some(v) if v == value => Err(()),
                _ => Ok(()),
            }
        }

        pub fn set(&mut self, value: T) -> Result<(), ()> {
            match self.compare_exchange(&value) {
                Ok(_) => {
                    self.value = Some(value);
                    Ok(())
                },
                Err(_) => Err(()),
            }
        }
    }

    let mut cell = Cell::new();
    cell.set(42).unwrap(); // Initialize the cell with a value
    // Now, the cell is full, so setting the same value should return Err(())
    let result = cell.set(42);
    assert_eq!(result, Err(()));
}

#[test]
fn test_set_with_different_value_when_cell_is_full() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T: PartialEq> Cell<T> {
        pub fn new() -> Self {
            Cell { value: None }
        }

        pub fn compare_exchange(&self, value: &T) -> Result<(), ()> {
            match &self.value {
                Some(_) => Err(()),
                _ => Ok(()),
            }
        }

        pub fn set(&mut self, value: T) -> Result<(), ()> {
            match self.compare_exchange(&value) {
                Ok(_) => {
                    self.value = Some(value);
                    Ok(())
                },
                Err(_) => Err(()),
            }
        }
    }

    let mut cell = Cell::new();
    cell.set(42).unwrap(); // Initialize the cell with a value
    // Now, the cell should return Err(()) for any value since it is already set
    let result = cell.set(99);
    assert_eq!(result, Err(()));
}

