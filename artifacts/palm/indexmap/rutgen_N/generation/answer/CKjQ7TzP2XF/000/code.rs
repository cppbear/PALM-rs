// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct TestMap {
        entries: Vec<(String, usize)>,
        indices: HashMap<String, usize>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: HashMap::new(),
            }
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.indices
                .try_reserve(additional, self.entries.len())
                .map_err(TryReserveError::from_hashbrown)?;
            self.entries
                .try_reserve_exact(additional)
                .map_err(TryReserveError::from_alloc)
        }
    }

    #[test]
    fn test_try_reserve_exact_success() {
        let mut map = TestMap::new();
        assert!(map.try_reserve_exact(5).is_ok());
        assert_eq!(map.entries.capacity(), 5);
    }

    #[test]
    fn test_try_reserve_exact_zero() {
        let mut map = TestMap::new();
        assert!(map.try_reserve_exact(0).is_ok());
        assert_eq!(map.entries.capacity(), 0);
    }

    #[test]
    #[should_panic]
    fn test_try_reserve_exact_overflow() {
        let mut map = TestMap::new();
        let _ = map.try_reserve_exact(usize::MAX);
    }
}

