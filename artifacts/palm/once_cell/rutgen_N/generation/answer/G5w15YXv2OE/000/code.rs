// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::sync::Arc;
    use std::sync::Mutex;

    struct TestCell {
        value: Arc<Mutex<Option<NonZeroUsize>>>,
    }

    impl TestCell {
        pub fn new() -> Self {
            TestCell {
                value: Arc::new(Mutex::new(None)),
            }
        }

        pub fn compare_exchange(&self, new_value: NonZeroUsize) -> Result<(), ()> {
            let mut lock = self.value.lock().unwrap();
            if lock.is_none() {
                *lock = Some(new_value);
                Ok(())
            } else {
                Err(())
            }
        }

        pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    #[test]
    fn test_set_empty_cell() {
        let cell = TestCell::new();
        let value = NonZeroUsize::new(1).unwrap();
        assert_eq!(cell.set(value), Ok(()));
    }

    #[test]
    fn test_set_full_cell() {
        let cell = TestCell::new();
        let value1 = NonZeroUsize::new(1).unwrap();
        let value2 = NonZeroUsize::new(2).unwrap();
        assert_eq!(cell.set(value1), Ok(()));
        assert_eq!(cell.set(value2), Err(()));
    }

    #[test]
    fn test_set_with_zero() {
        let cell = TestCell::new();
        assert!(NonZeroUsize::new(0).is_none());
    }
}

