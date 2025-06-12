// Answer 0

#[test]
fn test_get_or_init_empty_cell() {
    struct Cell {
        initialized: bool,
        value: bool,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                initialized: false,
                value: false,
            }
        }
        
        pub fn get_or_init<F>(&mut self, f: F) -> bool
        where
            F: FnOnce() -> bool,
        {
            if !self.initialized {
                self.value = f();
                self.initialized = true;
            }
            self.value
        }
    }

    let mut cell = Cell::new();
    
    let result = cell.get_or_init(|| true);
    
    assert_eq!(result, true);
    assert_eq!(cell.initialized, true);
}

#[test]
fn test_get_or_init_cell_already_initialized() {
    struct Cell {
        initialized: bool,
        value: bool,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                initialized: false,
                value: false,
            }
        }
        
        pub fn get_or_init<F>(&mut self, f: F) -> bool
        where
            F: FnOnce() -> bool,
        {
            if !self.initialized {
                self.value = f();
                self.initialized = true;
            }
            self.value
        }
    }

    let mut cell = Cell::new();
    let _ = cell.get_or_init(|| false); // Initialize first time

    let result = cell.get_or_init(|| true); // Should not call f again
    
    assert_eq!(result, false);
    assert_eq!(cell.initialized, true);
}

#[test]
#[should_panic]
fn test_get_or_init_panic_condition() {
    struct Cell {
        initialized: bool,
        value: bool,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                initialized: false,
                value: false,
            }
        }
        
        pub fn get_or_init<F>(&mut self, f: F) -> bool
        where
            F: FnOnce() -> bool,
        {
            if !self.initialized {
                self.value = f();
                self.initialized = true;
            }
            self.value
        }
    }

    let mut cell = Cell::new();
    
    // This closure will panic
    let _ = cell.get_or_init(|| panic!("panic test"));
}

