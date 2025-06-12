// Answer 0

#[test]
fn test_replace_full_insert_new_entry() {
    struct TestMap {
        core: IndexMapCore<usize, usize>
    }
    
    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn replace_full(&mut self, hash: HashValue, key: usize, value: usize) -> (usize, Option<(usize, usize)>) {
            self.core.replace_full(hash, key, value)
        }
    }
    
    let mut map = TestMap::new();
    let hash = HashValue(1);
    let (index, previous) = map.replace_full(hash, 10, 100);
    
    assert_eq!(index, 0);
    assert_eq!(previous, None);
}

#[test]
fn test_replace_full_update_existing_entry() {
    struct TestMap {
        core: IndexMapCore<usize, usize>
    }
    
    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn replace_full(&mut self, hash: HashValue, key: usize, value: usize) -> (usize, Option<(usize, usize)>) {
            self.core.replace_full(hash, key, value)
        }
    }
    
    let mut map = TestMap::new();
    let hash = HashValue(1);
    
    // Insert new entry first
    map.replace_full(hash, 10, 100);
    
    // Replace existing entry
    let (index, previous) = map.replace_full(hash, 10, 200);
    
    assert_eq!(index, 0);
    assert_eq!(previous, Some((10, 100)));
}

#[test]
#[should_panic]
fn test_replace_full_fail_on_invalid_hash() {
    struct TestMap {
        core: IndexMapCore<usize, usize>
    }
    
    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn replace_full(&mut self, hash: HashValue, key: usize, value: usize) -> (usize, Option<(usize, usize)>) {
            self.core.replace_full(hash, key, value)
        }
    }
    
    let mut map = TestMap::new();
    let hash = HashValue(2); // Use a different hash to trigger panic for vacuum entry access
    let (index, previous) = map.replace_full(hash, 10, 100);
    assert_eq!(index, 0);
}

