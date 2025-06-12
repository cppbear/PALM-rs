// Answer 0

#[test]
fn test_set_success() {
    use std::num::NonZeroUsize;
    use std::sync::Mutex;

    struct Cell {
        value: Mutex<Option<NonZeroUsize>>,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                value: Mutex::new(None),
            }
        }

        fn compare_exchange(&self, new_value: NonZeroUsize) -> Result<(), ()> {
            let mut lock = self.value.lock().unwrap();
            if lock.is_none() {
                *lock = Some(new_value);
                Ok(())
            } else {
                Err(())
            }
        }

        fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let cell = Cell::new();
    let value = NonZeroUsize::new(1).unwrap();
    
    assert_eq!(cell.set(value), Ok(()));
} 

#[test]
fn test_set_fail_already_set() {
    use std::num::NonZeroUsize;
    use std::sync::Mutex;

    struct Cell {
        value: Mutex<Option<NonZeroUsize>>,
    }

    impl Cell {
        fn new() -> Self {
            Cell {
                value: Mutex::new(None),
            }
        }

        fn compare_exchange(&self, new_value: NonZeroUsize) -> Result<(), ()> {
            let mut lock = self.value.lock().unwrap();
            if lock.is_none() {
                *lock = Some(new_value);
                Ok(())
            } else {
                Err(())
            }
        }

        fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let cell = Cell::new();
    let value1 = NonZeroUsize::new(1).unwrap();
    let value2 = NonZeroUsize::new(2).unwrap();
    
    assert_eq!(cell.set(value1), Ok(()));
    assert_eq!(cell.set(value2), Err(()));
}

