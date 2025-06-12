// Answer 0

#[test]
fn test_set_success() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Cell {
        value: AtomicUsize,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                value: AtomicUsize::new(0),
            }
        }

        fn compare_exchange(&self, value: NonZeroUsize) -> Result<NonZeroUsize, NonZeroUsize> {
            let current = self.value.load(Ordering::SeqCst);
            if current == 0 {
                self.value.store(value.get(), Ordering::SeqCst);
                Ok(value)
            } else {
                Err(value)
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
    let non_zero_value = NonZeroUsize::new(1).unwrap();
    
    assert_eq!(cell.set(non_zero_value), Ok(()));
}

#[test]
fn test_set_fail_full_cell() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Cell {
        value: AtomicUsize,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                value: AtomicUsize::new(1),
            }
        }

        fn compare_exchange(&self, value: NonZeroUsize) -> Result<NonZeroUsize, NonZeroUsize> {
            let current = self.value.load(Ordering::SeqCst);
            if current == 0 {
                self.value.store(value.get(), Ordering::SeqCst);
                Ok(value)
            } else {
                Err(value)
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
    let non_zero_value = NonZeroUsize::new(2).unwrap();
    
    assert_eq!(cell.set(non_zero_value), Err(()));
}

