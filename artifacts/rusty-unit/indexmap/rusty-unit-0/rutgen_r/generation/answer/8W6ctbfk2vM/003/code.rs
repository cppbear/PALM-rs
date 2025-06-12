// Answer 0

#[test]
fn test_try_reserve_entries_exact_capacity() {
    use indexmap::IndexMap;
    use std::alloc::Layout;
    use std::error::Error;
    
    struct TryReserveError;

    impl TryReserveError {
        fn from_alloc(_layout: Layout) -> Self {
            TryReserveError
        }
    }

    struct TestMap {
        entries: IndexMap<i32, i32>,
        indices: IndexMap<i32, i32>,
    }

    impl TestMap {
        const MAX_ENTRIES_CAPACITY: usize = 10;

        fn new() -> Self {
            TestMap {
                entries: IndexMap::new(),
                indices: IndexMap::new(),
            }
        }

        fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
            let new_capacity = std::cmp::min(self.indices.capacity(), Self::MAX_ENTRIES_CAPACITY);
            let try_add = new_capacity - self.entries.len();
            if try_add > additional && self.entries.try_reserve_exact(try_add).is_ok() {
                return Ok(());
            }
            self.entries
                .try_reserve_exact(additional)
                .map_err(TryReserveError::from_alloc)
        }
    }

    let mut map = TestMap::new();
    map.indices.insert(0, 0); // Make the capacity of indices 1

    // Ensure entries have 0 capacity
    assert_eq!(map.entries.len(), 0);
    // Reserve capacity exactly equal to the allowed capacity
    let result = map.try_reserve_entries(1); 
    assert!(result.is_ok());
    assert_eq!(map.entries.len(), 1);
}

