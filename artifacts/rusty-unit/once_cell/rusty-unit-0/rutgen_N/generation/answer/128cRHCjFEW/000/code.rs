// Answer 0

#[test]
fn test_set_success() {
    struct Cell {
        inner: once_cell::cell::UnsafeCell<Option<usize>>,
    }

    impl Cell {
        pub fn new() -> Self {
            Cell {
                inner: once_cell::cell::UnsafeCell::new(None),
            }
        }
        
        pub fn to_usize(value: bool) -> usize {
            if value { 1 } else { 0 }
        }
        
        pub fn set(&self, value: bool) -> Result<(), ()> {
            let mut inner = unsafe { &mut *self.inner.get() };
            if inner.is_none() {
                *inner = Some(Self::to_usize(value));
                Ok(())
            } else {
                Err(())
            }
        }
    }

    let cell = Cell::new();
    assert_eq!(cell.set(true), Ok(()));
    assert_eq!(cell.set(false), Err(()));
}

#[test]
fn test_set_failure() {
    struct Cell {
        inner: once_cell::cell::UnsafeCell<Option<usize>>,
    }

    impl Cell {
        pub fn new() -> Self {
            Cell {
                inner: once_cell::cell::UnsafeCell::new(None),
            }
        }
        
        pub fn to_usize(value: bool) -> usize {
            if value { 1 } else { 0 }
        }
        
        pub fn set(&self, value: bool) -> Result<(), ()> {
            let mut inner = unsafe { &mut *self.inner.get() };
            if inner.is_none() {
                *inner = Some(Self::to_usize(value));
                Ok(())
            } else {
                Err(())
            }
        }
    }

    let cell = Cell::new();
    assert_eq!(cell.set(true), Ok(()));
    assert_eq!(cell.set(false), Err(()));
    assert_eq!(cell.set(true), Err(())); // testing again after initial full
}

