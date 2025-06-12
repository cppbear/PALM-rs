// Answer 0

#[test]
fn test_set_when_cell_is_full() {
    use std::num::NonZeroUsize;

    struct Cell {
        value: Option<NonZeroUsize>,
    }

    impl Cell {
        fn new() -> Self {
            Cell { value: None }
        }

        fn compare_exchange(&self, value: NonZeroUsize) -> Result<(), ()> {
            if self.value.is_some() {
                Err(())
            } else {
                Ok(())
            }
        }

        pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let cell = Cell {
        value: Some(NonZeroUsize::new(1).unwrap()),
    };

    let result = cell.set(NonZeroUsize::new(2).unwrap());
    assert_eq!(result, Err(()));
}

#[test]
fn test_set_when_cell_is_empty() {
    use std::num::NonZeroUsize;

    struct Cell {
        value: Option<NonZeroUsize>,
    }

    impl Cell {
        fn new() -> Self {
            Cell { value: None }
        }

        fn compare_exchange(&self, value: NonZeroUsize) -> Result<(), ()> {
            if self.value.is_some() {
                Err(())
            } else {
                Ok(())
            }
        }

        pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let cell = Cell::new();

    let result = cell.set(NonZeroUsize::new(2).unwrap());
    assert_eq!(result, Ok(()));
}

