// Answer 0

#[test]
fn test_get_or_try_init_with_initialized_cell() {
    struct MyCell {
        data: Option<i32>,
    }

    impl MyCell {
        fn new() -> Self {
            MyCell { data: None }
        }

        fn get(&self) -> Option<&i32> {
            self.data.as_ref()
        }

        fn initialize<F, E>(&mut self, f: F) -> Result<(), E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if self.data.is_none() {
                self.data = Some(f()?);
            }
            Ok(())
        }

        fn is_initialized(&self) -> bool {
            self.data.is_some()
        }

        unsafe fn get_unchecked(&self) -> &i32 {
            self.data.as_ref().unwrap()
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(value) = self.get() {
                return Ok(value);
            }

            self.initialize(f)?;

            debug_assert!(self.is_initialized());
            Ok(unsafe { self.get_unchecked() })
        }
    }

    let mut cell = MyCell::new();
    
    // Initialize the cell successfully
    let value = cell.get_or_try_init(|| Ok(42));
    assert_eq!(value, Ok(unsafe { &42 }));
    assert_eq!(cell.get(), Some(&42));
    
    // Simulate the cell being already initialized
    let value_after_initialization = cell.get_or_try_init(|| Ok(99));
    assert_eq!(value_after_initialization, Ok(unsafe { &42 }));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
#[should_panic]
fn test_get_or_try_init_panics_on_get_unchecked() {
    struct PanicCell {
        data: Option<i32>,
    }

    impl PanicCell {
        fn new() -> Self {
            PanicCell { data: None }
        }

        fn get(&self) -> Option<&i32> {
            self.data.as_ref()
        }

        fn initialize<F, E>(&mut self, f: F) -> Result<(), E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if self.data.is_none() {
                self.data = Some(f()?);
            }
            Ok(())
        }

        fn is_initialized(&self) -> bool {
            self.data.is_some()
        }

        unsafe fn get_unchecked(&self) -> &i32 {
            if self.data.is_none() {
                panic!("Accessing uninitialized cell!");
            }
            self.data.as_ref().unwrap()
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(value) = self.get() {
                return Ok(value);
            }

            self.initialize(f)?;

            debug_assert!(self.is_initialized());
            Ok(unsafe { self.get_unchecked() })
        }
    }

    let mut cell = PanicCell::new();
    
    // Trigger panic by calling get_unchecked when the cell is uninitialized
    let _ = cell.get_or_try_init(|| Err(()));
}

