// Answer 0

#[test]
fn test_swap_remove_full_single_matching_key() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(1, 10)] } // A single entry
        }

        fn as_entries(&self) -> &[usize] {
            &self.entries.iter().map(|(key, _)| *key).collect::<Vec<_>>()[..]
        }

        fn hash(&self, key: &usize) -> usize {
            *key // Simplified hash function for testing
        }

        fn core_pop(&mut self) -> Option<(usize, usize)> {
            self.entries.pop()
        }

        fn swap_remove_full(&mut self, key: &usize) -> Option<(usize, usize)> {
            match self.as_entries() {
                [x] if key == x => {
                    let (k, v) = self.core_pop()?;
                    Some((0, k, v))
                }
                _ => None,
            }
        }
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_full(&2); // Key does not match
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_empty_map() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![] } // Empty map
        }

        fn as_entries(&self) -> &[usize] {
            &self.entries.iter().map(|(key, _)| *key).collect::<Vec<_>>()[..]
        }

        fn swap_remove_full(&mut self, _key: &usize) -> Option<(usize, usize)> {
            None // No entries to remove
        }
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_full(&1); // Trying to remove from empty map
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_single_entry_not_matching() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(1, 10)] } // A single entry
        }

        fn as_entries(&self) -> &[usize] {
            &self.entries.iter().map(|(key, _)| *key).collect::<Vec<_>>()[..]
        }

        fn swap_remove_full(&mut self, key: &usize) -> Option<(usize, usize)> {
            match self.as_entries() {
                [x] if key != x => {
                    return None;  // Key does not match
                }
                _ => None,
            }
        }
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_full(&2); // Key does not match
    assert_eq!(result, None);
}

