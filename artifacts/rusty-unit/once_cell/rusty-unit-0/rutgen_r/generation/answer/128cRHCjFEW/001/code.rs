// Answer 0

#[test]
fn test_set_success() {
    struct Cell {
        inner: once_cell::unsync::OnceCell<usize>,
    }
    
    impl Cell {
        fn to_usize(value: bool) -> usize {
            if value {
                1
            } else {
                0
            }
        }

        pub fn set(&self, value: bool) -> Result<(), ()> {
            self.inner.set(Self::to_usize(value))
        }
    }

    let cell = Cell {
        inner: once_cell::unsync::OnceCell::new(),
    };

    // Test case: Setting the cell with a value when it's empty
    assert_eq!(cell.set(true), Ok(()));
    // Test case: Setting the cell with another value should fail (it's already full)
    assert_eq!(cell.set(false), Err(()));
}

#[test]
fn test_set_full_cell() {
    struct Cell {
        inner: once_cell::unsync::OnceCell<usize>,
    }
    
    impl Cell {
        fn to_usize(value: bool) -> usize {
            if value {
                1
            } else {
                0
            }
        }

        pub fn set(&self, value: bool) -> Result<(), ()> {
            self.inner.set(Self::to_usize(value))
        }
    }

    let cell = Cell {
        inner: once_cell::unsync::OnceCell::new(),
    };

    // Fill the cell initially
    let _ = cell.set(true);

    // Test case: Attempting to set a different value should return Err
    assert_eq!(cell.set(true), Err(()));
    assert_eq!(cell.set(false), Err(()));
}

#[test]
fn test_set_empty_cell() {
    struct Cell {
        inner: once_cell::unsync::OnceCell<usize>,
    }
    
    impl Cell {
        fn to_usize(value: bool) -> usize {
            if value {
                1
            } else {
                0
            }
        }

        pub fn set(&self, value: bool) -> Result<(), ()> {
            self.inner.set(Self::to_usize(value))
        }
    }

    let cell = Cell {
        inner: once_cell::unsync::OnceCell::new(),
    };

    // Test case: Initially empty, setting it to false should succeed.
    assert_eq!(cell.set(false), Ok(()));
    // Test case: Setting to true should succeed as well.
    assert_eq!(cell.set(true), Err(()));
}

