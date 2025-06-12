// Answer 0

#[test]
fn test_set_empty_cell() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        pub fn new() -> Self {
            Cell { value: None }
        }

        pub fn compare_exchange(&self, value: &T) -> Result<(), ()> {
            if self.value.is_none() {
                return Ok(());
            }
            Err(())
        }

        pub fn set(&self, value: &'_ T) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => {
                    self.value = Some(value.clone());
                    Ok(())
                },
                Err(_) => Err(()),
            }
        }
    }

    let cell = Cell::new();
    let result = cell.set(&42);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_set_full_cell() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        pub fn new() -> Self {
            Cell { value: None }
        }

        pub fn compare_exchange(&self, _value: &T) -> Result<(), ()> {
            if self.value.is_none() {
                return Ok(());
            }
            Err(())
        }

        pub fn set(&mut self, value: &'_ T) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => {
                    self.value = Some(value.clone());
                    Ok(())
                },
                Err(_) => Err(()),
            }
        }
    }

    let mut cell = Cell::new();
    let _ = cell.set(&42);
    let result = cell.set(&99);
    assert_eq!(result, Err(()));
}

