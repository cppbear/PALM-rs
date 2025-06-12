// Answer 0

#[test]
fn test_get_or_try_init_with_initialized_value() {
    struct TestOnceCell {
        cell: OnceCell<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell {
                cell: OnceCell::new(),
            }
        }
        
        fn initialize(&self, value: i32) -> Result<&i32, ()> {
            self.cell.set(value).map_err(|_| ())
        }
    }

    let once_cell = TestOnceCell::new();
    assert!(once_cell.cell.get().is_none());
    
    let result = once_cell.initialize(42);
    assert!(result.is_ok());
    assert_eq!(once_cell.cell.get(), Some(&42));
    
    let second_result = once_cell.cell.get_or_try_init(|| Ok(100));
    assert_eq!(second_result, Ok(&42)); // Already initialized, should return 42
}

#[test]
fn test_get_or_try_init_with_failed_initialization() {
    struct TestOnceCell {
        cell: OnceCell<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell {
                cell: OnceCell::new(),
            }
        }
        
        fn fail_initialize(&self) -> Result<&i32, ()> {
            Err(())
        }
    }

    let once_cell = TestOnceCell::new();
    assert!(once_cell.cell.get().is_none());
    
    let result = once_cell.cell.get_or_try_init(|| once_cell.fail_initialize());
    assert_eq!(result, Err(())); // Initialization failed
    assert!(once_cell.cell.get().is_none());
}

#[test]
#[should_panic]
fn test_get_or_try_init_panics_on_reentrant_initialization() {
    struct TestOnceCell {
        cell: OnceCell<i32>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell {
                cell: OnceCell::new(),
            }
        }
        
        fn reentrant_init(&self) -> Result<i32, ()> {
            self.cell.get_or_try_init(|| self.reentrant_init()).map(|_| 0)
        }
    }

    let once_cell = TestOnceCell::new();
    let _ = once_cell.cell.get_or_try_init(|| once_cell.reentrant_init());
}

