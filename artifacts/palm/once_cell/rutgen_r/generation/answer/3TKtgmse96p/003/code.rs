// Answer 0

#[test]
fn test_get_or_try_init_with_initialized_value() {
    struct OnceCellWrapper {
        value: Option<i32>,
        initialized: bool,
    }

    impl OnceCellWrapper {
        fn new() -> Self {
            OnceCellWrapper { value: None, initialized: false }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn initialize<E, F>(&mut self, f: F) -> Result<(), E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if self.initialized {
                return Err(());
            }
            self.value = Some(f()?);
            self.initialized = true;
            Ok(())
        }

        fn is_initialized(&self) -> bool {
            self.initialized
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(value) = self.get() {
                return Ok(value);
            }
            self.initialize(f)?;

            Ok(self.get().unwrap())
        }
    }

    let mut cell = OnceCellWrapper::new();

    // Initialize with a valid result
    let result = cell.get_or_try_init(|| Ok(42));
    assert_eq!(result, Ok(&42));
    assert!(cell.is_initialized());
    
    // Verify that the cell returns the initialized value
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_with_error() {
    struct OnceCellWrapper {
        value: Option<i32>,
        initialized: bool,
    }

    impl OnceCellWrapper {
        fn new() -> Self {
            OnceCellWrapper { value: None, initialized: false }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn initialize<E, F>(&mut self, f: F) -> Result<(), E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if self.initialized {
                return Err(());
            }
            self.value = Some(f()?);
            self.initialized = true;
            Ok(())
        }

        fn is_initialized(&self) -> bool {
            self.initialized
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(value) = self.get() {
                return Ok(value);
            }
            self.initialize(f)?;

            Ok(self.get().unwrap())
        }
    }

    let mut cell = OnceCellWrapper::new();

    // Try to initialize with an error
    let result = cell.get_or_try_init(|| Err(()));
    assert_eq!(result, Err(()));
    assert!(cell.get().is_none());
}

#[test]
#[should_panic]
fn test_get_or_try_init_panic() {
    struct OnceCellWrapper {
        value: Option<i32>,
        initialized: bool,
    }

    impl OnceCellWrapper {
        fn new() -> Self {
            OnceCellWrapper { value: None, initialized: false }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }

        fn initialize<E, F>(&mut self, f: F) -> Result<(), E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if self.initialized {
                return Err(());
            }
            self.value = Some(f()?);
            self.initialized = true;
            Ok(())
        }

        fn is_initialized(&self) -> bool {
            self.initialized
        }

        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&i32, E>
        where
            F: FnOnce() -> Result<i32, E>,
        {
            if let Some(value) = self.get() {
                return Ok(value);
            }
            self.initialize(f)?;

            Ok(self.get().unwrap())
        }
    }

    let mut cell = OnceCellWrapper::new();

    // This will panic
    cell.get_or_try_init(|| panic!("Panic for testing"));
}

