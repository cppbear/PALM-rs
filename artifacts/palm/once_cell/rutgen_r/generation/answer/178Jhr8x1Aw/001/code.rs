// Answer 0

#[test]
fn test_try_insert_err_case() {
    struct TestCell {
        value: Option<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn try_insert(&mut self, value: i32) -> Result<&i32, (&i32, i32)> {
            if let Some(ref existing) = self.value {
                Err((existing, value))
            } else {
                self.value = Some(value);
                Ok(self.value.as_ref().unwrap())
            }
        }
    }

    let mut cell = TestCell::new();
    
    assert!(cell.get().is_none());
    
    // First insertion should succeed
    assert_eq!(cell.try_insert(92), Ok(&92));
    
    // Attempting a second insertion should fail and return the expected error
    assert_eq!(cell.try_insert(62), Err((&92, 62)));
    
    // Verify that the value is still present in the cell
    assert!(cell.get().is_some());
}

