// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use once_cell::sync::Lazy;

    struct TestCell {
        value: Option<NonZeroUsize>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell { value: None }
        }

        fn get(&self) -> Option<NonZeroUsize> {
            self.value
        }

        fn init<F, E>(&mut self, f: F) -> Result<NonZeroUsize, E>
        where
            F: FnOnce() -> Result<NonZeroUsize, E>,
        {
            let result = f()?;
            self.value = Some(result);
            Ok(result)
        }
    }

    impl TestCell {
        fn get_or_try_init<F, E>(&mut self, f: F) -> Result<NonZeroUsize, E>
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
    fn test_get_or_try_init_empty_cell() {
        let mut cell = TestCell::new();
        let result = cell.get_or_try_init(|| NonZeroUsize::new(42).ok_or("failed"));
        assert_eq!(result, Ok(NonZeroUsize::new(42).unwrap()));
    }

    #[test]
    fn test_get_or_try_init_non_empty_cell() {
        let mut cell = TestCell::new();
        let _ = cell.get_or_try_init(|| NonZeroUsize::new(42).ok_or("failed"));
        let result = cell.get_or_try_init(|| NonZeroUsize::new(30).ok_or("failed"));
        assert_eq!(result, Ok(NonZeroUsize::new(42).unwrap()));
    }

    #[test]
    #[should_panic(expected = "failed")]
    fn test_get_or_try_init_initialization_failure() {
        let mut cell = TestCell::new();
        let _ = cell.get_or_try_init(|| Err("failed"));
    }
}

