// Answer 0

#[test]
fn test_swap_remove_full_existing_key() {
    struct TestIndexMap {
        core: IndexMapCore<i32, String>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                core: IndexMapCore::new(),
            }
        }
        
        fn insert(&mut self, key: i32, value: String) {
            let hash = HashValue(0); // Simulated hash
            self.core.push_entry(hash, key, value);
        }
        
        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.core.entries
        }
        
        fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> HashValue {
            HashValue(0) // Simulated hash
        }
    }

    let mut map = TestIndexMap::new();
    map.insert(10, "ten".to_string());
    map.insert(20, "twenty".to_string());
    
    if let Some((index, value)) = map.swap_remove_full(&10) {
        assert_eq!(index, 0);
        assert_eq!(value, "ten");
    } else {
        panic!("Expected to remove the key");
    }
}

#[test]
fn test_swap_remove_full_non_existing_key() {
    struct TestIndexMap {
        core: IndexMapCore<i32, String>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                core: IndexMapCore::new(),
            }
        }
        
        fn insert(&mut self, key: i32, value: String) {
            let hash = HashValue(0); // Simulated hash
            self.core.push_entry(hash, key, value);
        }
        
        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.core.entries
        }
        
        fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> HashValue {
            HashValue(0) // Simulated hash
        }
    }

    let mut map = TestIndexMap::new();
    map.insert(10, "ten".to_string());
    map.insert(20, "twenty".to_string());
    
    assert!(map.swap_remove_full(&30).is_none());
}

#[test]
fn test_swap_remove_full_last_element() {
    struct TestIndexMap {
        core: IndexMapCore<i32, String>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                core: IndexMapCore::new(),
            }
        }
        
        fn insert(&mut self, key: i32, value: String) {
            let hash = HashValue(0); // Simulated hash
            self.core.push_entry(hash, key, value);
        }
        
        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.core.entries
        }
        
        fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> HashValue {
            HashValue(0) // Simulated hash
        }
    }

    let mut map = TestIndexMap::new();
    map.insert(10, "ten".to_string());
    
    if let Some((index, value)) = map.swap_remove_full(&10) {
        assert_eq!(index, 0);
        assert_eq!(value, "ten");
    } else {
        panic!("Expected to remove the key");
    }
}

#[test]
fn test_swap_remove_full_empty_map() {
    struct TestIndexMap {
        core: IndexMapCore<i32, String>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                core: IndexMapCore::new(),
            }
        }
        
        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.core.entries
        }
        
        fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> HashValue {
            HashValue(0) // Simulated hash
        }
    }

    let mut map = TestIndexMap::new();
    
    assert!(map.swap_remove_full(&10).is_none());
}

