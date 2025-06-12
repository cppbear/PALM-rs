// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
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

        fn get(&self) -> Option<NonZeroUsize> {
            let value = self.value.lock().unwrap();
            *value
        }

        fn init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>
        where
            F: FnOnce() -> Result<NonZeroUsize, E>,
        {
            let mut value = self.value.lock().unwrap();
            if value.is_none() {
                match f() {
                    Ok(v) => {
                        *value = Some(v);
                        Ok(v)
                    },
                    Err(e) => Err(e),
                }
            } else {
                Ok(value.unwrap())
            }
        }

        pub fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>
        where
            F: FnOnce() -> Result<NonZeroUsize, E>,
        {
            match self.get() {
                Some(it) => Ok(it),
                None => self.init(f),
            }
        }
    }

    #[test]
    fn test_get_or_try_init_with_some_value() {
        let cell = Cell::new();
        let init_value: NonZeroUsize = NonZeroUsize::new(42).unwrap();
        let result = cell.get_or_try_init(|| Ok(init_value));
        assert_eq!(result, Ok(init_value));
    }

    #[test]
    fn test_get_or_try_init_returns_same_value() {
        let cell = Cell::new();
        let init_value: NonZeroUsize = NonZeroUsize::new(99).unwrap();
        let _ = cell.get_or_try_init(|| Ok(init_value));
        
        let result = cell.get_or_try_init(|| Ok(NonZeroUsize::new(1).unwrap()));
        assert_eq!(result, Ok(init_value));
    }

    #[test]
    fn test_get_or_try_init_when_already_initialized() {
        let cell = Cell::new();
        let init_value: NonZeroUsize = NonZeroUsize::new(7).unwrap();
        let _ = cell.get_or_try_init(|| Ok(init_value));

        let result = cell.get_or_try_init(|| panic!("Should not initialize again"));
        assert_eq!(result, Ok(init_value));
    }
}

