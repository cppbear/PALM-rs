// Answer 0

#[test]
fn test_get_or_try_init_success_with_preinitialized_cell() {
    struct CustomCell {
        value: Option<i32>,
    }
    
    impl CustomCell {
        fn new() -> Self {
            CustomCell { value: None }
        }
        
        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
        
        fn set(&mut self, val: i32) -> Result<(), &'static str> {
            if self.value.is_some() {
                Err("Cell is already initialized.")
            } else {
                self.value = Some(val);
                Ok(())
            }
        }
        
        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(self.get().unwrap())
        }
    }

    let mut cell = CustomCell::new();
    let init_result = cell.get_or_try_init(|| Ok(42));
    assert_eq!(init_result, Ok(&42));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
#[should_panic(expected = "Cell is already initialized.")]
fn test_get_or_try_init_panic_on_reentrant_init() {
    struct CustomCell {
        value: Option<i32>,
    }
    
    impl CustomCell {
        fn new() -> Self {
            CustomCell { value: None }
        }
        
        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
        
        fn set(&mut self, val: i32) -> Result<(), &'static str> {
            if self.value.is_some() {
                Err("Cell is already initialized.")
            } else {
                self.value = Some(val);
                Ok(())
            }
        }
        
        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(val) = self.get() {
                return Ok(val);
            }
            let val = f()?;
            assert!(self.set(val).is_ok(), "reentrant init");
            Ok(self.get().unwrap())
        }
    }

    let mut cell = CustomCell::new();
    let _ = cell.get_or_try_init(|| Ok(10));
    let _ = cell.get_or_try_init(|| Ok(20)); // This should panic due to reentrant initialization.
}

