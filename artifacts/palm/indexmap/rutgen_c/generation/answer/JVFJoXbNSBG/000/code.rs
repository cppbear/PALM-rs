// Answer 0

#[test]
fn test_split_off_boundary() {
    struct TestMap {
        core: IndexMapCore<u32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn len(&self) -> usize {
            self.core.len()
        }

        fn capacity(&self) -> usize {
            self.core.capacity()
        }

        fn split_off(&mut self, at: usize) -> Self {
            let len = self.len();
            assert!(at <= len, "index out of bounds");
            TestMap {
                core: self.core.split_off(at),
            }
        }
    }

    let mut map = TestMap::new();
    // Here we would normally populate the map, but for this test case let's assume it's empty.
    
    // Test split off at index 0
    let split = map.split_off(0);
    assert_eq!(split.len(), 0);
    assert_eq!(map.len(), 0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_off_panics_on_out_of_bounds() {
    struct TestMap {
        core: IndexMapCore<u32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn len(&self) -> usize {
            self.core.len()
        }

        fn split_off(&mut self, at: usize) -> Self {
            let len = self.len();
            assert!(at <= len, "index out of bounds");
            TestMap {
                core: self.core.split_off(at),
            }
        }
    }

    let mut map = TestMap::new();
    // Here we would normally populate the map, but for this test case let's assume it's empty.
    
    // Attempt to split off at an out-of-bounds index
    let _ = map.split_off(1); // This should panic
}

#[test]
fn test_split_off_with_some_elements() {
    struct TestMap {
        core: IndexMapCore<u32, String>,
        entries: Vec<(u32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
                entries: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
            }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn split_off(&mut self, at: usize) -> Self {
            let len = self.len();
            assert!(at <= len, "index out of bounds");
            let new_entries = self.entries.split_off(at);
            TestMap {
                core: IndexMapCore::new(), // Assume a new core for the split map.
                entries: new_entries,
            }
        }

        fn get_entries(&self) -> &[(u32, String)] {
            &self.entries
        }
    }

    let mut map = TestMap::new();
    let split = map.split_off(2);
    assert_eq!(split.get_entries().len(), 1);
    assert_eq!(map.get_entries().len(), 2);
}

