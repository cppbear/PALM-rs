// Answer 0

#[test]
fn test_set_with_full_cell() {
    struct TestOnceCell {
        value: Option<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            Self { value: None }
        }

        fn try_insert(&mut self, new_value: i32) -> Result<(), ((), i32)> {
            match &self.value {
                None => {
                    self.value = Some(new_value);
                    Ok(())
                }
                Some(existing_value) => Err(((), *existing_value)),
            }
        }

        fn set(&mut self, value: i32) -> Result<(), i32> {
            match self.try_insert(value) {
                Ok(_) => Ok(()),
                Err((_, existing_value)) => Err(existing_value),
            }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
    }

    let mut cell = TestOnceCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.set(92), Ok(()));
    assert_eq!(cell.set(62), Err(92)); // Expecting the value that was previously set to be returned.
    
    assert!(cell.get().is_some());
    assert_eq!(*cell.get().unwrap(), 92); // Check the value is indeed 92
}

#[test]
fn test_set_with_another_full_cell() {
    struct AnotherTestOnceCell {
        value: Option<i32>,
    }

    impl AnotherTestOnceCell {
        fn new() -> Self {
            Self { value: None }
        }

        fn try_insert(&mut self, new_value: i32) -> Result<(), ((), i32)> {
            match &self.value {
                None => {
                    self.value = Some(new_value);
                    Ok(())
                }
                Some(existing_value) => Err(((), *existing_value)),
            }
        }

        fn set(&mut self, value: i32) -> Result<(), i32> {
            match self.try_insert(value) {
                Ok(_) => Ok(()),
                Err((_, existing_value)) => Err(existing_value),
            }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
    }

    let mut cell = AnotherTestOnceCell::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.set(100), Ok(()));
    assert_eq!(cell.set(200), Err(100)); // Expecting the value that was previously set to be returned.

    assert!(cell.get().is_some());
    assert_eq!(*cell.get().unwrap(), 100); // Check the value is indeed 100
}

