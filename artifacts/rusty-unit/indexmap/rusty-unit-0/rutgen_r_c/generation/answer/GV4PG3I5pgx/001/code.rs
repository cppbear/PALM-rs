// Answer 0

#[test]
fn test_move_index_within_bounds() {
    struct MockIndexMap {
        core: IndexMapCore<i32, String>,
    }

    impl MockIndexMap {
        fn new() -> Self {
            MockIndexMap {
                core: IndexMapCore::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.core.entries.push((key, value));
            self.core.indices.push(self.core.entries.len() - 1);
        }
        
        fn move_index(&mut self, from: usize, to: usize) {
            assert!(from < self.core.entries.len() && to < self.core.entries.len());
            self.core.move_index(from, to);
        }
    }

    let mut map = MockIndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.move_index(0, 1);
    assert_eq!(map.core.entries[0], (2, "two".to_string()));
    assert_eq!(map.core.entries[1], (1, "one".to_string()));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_low() {
    struct MockIndexMap {
        core: IndexMapCore<i32, String>,
    }

    impl MockIndexMap {
        fn new() -> Self {
            MockIndexMap {
                core: IndexMapCore::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.core.entries.push((key, value));
            self.core.indices.push(self.core.entries.len() - 1);
        }
        
        fn move_index(&mut self, from: usize, to: usize) {
            assert!(from < self.core.entries.len() && to < self.core.entries.len());
            self.core.move_index(from, to);
        }
    }

    let mut map = MockIndexMap::new();
    map.insert(1, "one".to_string());

    // Should panic because from index is out of bounds
    map.move_index(1, 0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_high() {
    struct MockIndexMap {
        core: IndexMapCore<i32, String>,
    }

    impl MockIndexMap {
        fn new() -> Self {
            MockIndexMap {
                core: IndexMapCore::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.core.entries.push((key, value));
            self.core.indices.push(self.core.entries.len() - 1);
        }
        
        fn move_index(&mut self, from: usize, to: usize) {
            assert!(from < self.core.entries.len() && to < self.core.entries.len());
            self.core.move_index(from, to);
        }
    }

    let mut map = MockIndexMap::new();
    map.insert(1, "one".to_string());

    // Should panic because to index is out of bounds
    map.move_index(0, 1);
}

#[test]
fn test_move_index_reverse() {
    struct MockIndexMap {
        core: IndexMapCore<i32, String>,
    }

    impl MockIndexMap {
        fn new() -> Self {
            MockIndexMap {
                core: IndexMapCore::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.core.entries.push((key, value));
            self.core.indices.push(self.core.entries.len() - 1);
        }
        
        fn move_index(&mut self, from: usize, to: usize) {
            assert!(from < self.core.entries.len() && to < self.core.entries.len());
            self.core.move_index(from, to);
        }
    }

    let mut map = MockIndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.move_index(1, 0);
    assert_eq!(map.core.entries[0], (2, "two".to_string()));
    assert_eq!(map.core.entries[1], (1, "one".to_string()));
    assert_eq!(map.core.entries[2], (3, "three".to_string()));
}

