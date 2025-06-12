// Answer 0

#[test]
fn test_set_empty_cell() {
    struct Cell {
        inner: std::cell::Cell<Option<usize>>,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                inner: std::cell::Cell::new(None),
            }
        }

        fn to_usize(value: bool) -> usize {
            if value { 1 } else { 0 }
        }

        pub fn set(&self, value: bool) -> Result<(), ()> {
            if self.inner.get().is_none() {
                self.inner.set(Some(Self::to_usize(value)));
                Ok(())
            } else {
                Err(())
            }
        }
    }

    let cell = Cell::new();
    let result = cell.set(true);
    assert_eq!(result, Ok(()));
    assert_eq!(cell.inner.get(), Some(1));

    let result = cell.set(false);
    assert_eq!(result, Err(()));
}

#[test]
fn test_set_full_cell() {
    struct Cell {
        inner: std::cell::Cell<Option<usize>>,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                inner: std::cell::Cell::new(Some(1)),
            }
        }

        fn to_usize(value: bool) -> usize {
            if value { 1 } else { 0 }
        }

        pub fn set(&self, value: bool) -> Result<(), ()> {
            if self.inner.get().is_none() {
                self.inner.set(Some(Self::to_usize(value)));
                Ok(())
            } else {
                Err(())
            }
        }
    }

    let cell = Cell::new();
    let result = cell.set(true);
    assert_eq!(result, Err(()));
    assert_eq!(cell.inner.get(), Some(1));
}

