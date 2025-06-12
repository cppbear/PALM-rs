// Answer 0

#[test]
fn test_get_initialized_value() {
    struct TestOnceCell {
        value: u32,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell { value: 42 }
        }

        fn is_initialized(&self) -> bool {
            true // simulate initialized state
        }

        unsafe fn get_unchecked(&self) -> &u32 {
            &self.value // safely return reference to value
        }
    }

    struct OnceCellWrapper {
        cell: TestOnceCell,
    }

    impl OnceCellWrapper {
        pub const fn new() -> Self {
            OnceCellWrapper {
                cell: TestOnceCell::new(),
            }
        }

        pub fn get(&self) -> Option<&u32> {
            if self.cell.is_initialized() {
                Some(unsafe { self.cell.get_unchecked() })
            } else {
                None
            }
        }
    }

    let once_cell = OnceCellWrapper::new();
    assert_eq!(once_cell.get(), Some(&42));
}

#[test]
#[should_panic]
fn test_get_uninitialized_value() {
    struct TestOnceCell {
        initialized: bool,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell { initialized: false }
        }

        fn is_initialized(&self) -> bool {
            self.initialized
        }

        unsafe fn get_unchecked(&self) -> &u32 {
            panic!("Attempted to get an uninitialized value!");
        }
    }

    struct OnceCellWrapper {
        cell: TestOnceCell,
    }

    impl OnceCellWrapper {
        pub const fn new() -> Self {
            OnceCellWrapper {
                cell: TestOnceCell::new(),
            }
        }

        pub fn get(&self) -> Option<&u32> {
            if self.cell.is_initialized() {
                Some(unsafe { self.cell.get_unchecked() })
            } else {
                None
            }
        }
    }

    let once_cell = OnceCellWrapper::new();
    once_cell.get(); // This will trigger the panic in get_unchecked
}

