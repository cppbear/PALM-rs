// Answer 0

#[test]
fn test_get_or_try_init_with_some_value() {
    struct Cell<T> {
        value: Option<T>,
    }

    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }

        fn get(&self) -> Option<&T> {
            self.value.as_ref()
        }

        fn init<F, E>(&mut self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {
            match f() {
                Ok(v) => {
                    self.value = Some(v);
                    self.get().ok_or_else(|| panic!("Failed to initialize"))
                }
                Err(err) => Err(err),
            }
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
        {
            match self.get() {
                Some(val) => Ok(val),
                None => self.init(f),
            }
        }
    }

    let mut cell: Cell<i32> = Cell::new();
    
    assert_eq!(cell.get_or_try_init(|| Ok(42)), Ok(&42));
    assert_eq!(cell.get_or_try_init(|| Ok(13)), Ok(&42)); // Should return the same value
}

