// Answer 0

#[test]
fn test_swap_remove_entry() {
    struct TestMap {
        indices: Indices,
        entries: Entries<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                indices: Indices::new(),
                entries: Entries::new(),
            }
        }
        
        fn borrow_mut(&mut self) -> RefMut<i32, String> {
            RefMut::new(&mut self.indices, &mut self.entries)
        }
    }
  
    struct IndexMapCore {
        map: TestMap,
    }

    impl IndexMapCore {
        fn new() -> Self {
            Self {
                map: TestMap::new(),
            }
        }
        
        fn borrow_mut(&mut self) -> RefMut<i32, String> {
            self.map.borrow_mut()
        }
    }

    let mut index_map = IndexMapCore::new();
    let mut entry = IndexedEntry::new(&mut index_map, 0); // fake index for testing

    // Assume we have added an entry before we call swap_remove_entry
    entry.map.entries.push("Value".to_string()); // Simulate adding value for testing
    entry.map.indices.push(0); // Simulate adding index

    let (key, value) = entry.swap_remove_entry();
    assert_eq!(key, 0);
    assert_eq!(value, "Value".to_string());
}

#[test]
#[should_panic]
fn test_swap_remove_entry_empty() {
    struct TestMap {
        indices: Indices,
        entries: Entries<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                indices: Indices::new(),
                entries: Entries::new(),
            }
        }
    }

    struct IndexMapCore {
        map: TestMap,
    }

    impl IndexMapCore {
        fn new() -> Self {
            Self {
                map: TestMap::new(),
            }
        }
    }

    let mut index_map = IndexMapCore::new();
    let entry = IndexedEntry::new(&mut index_map, 0); // fake index for testing
    entry.swap_remove_entry(); // should panic because there is no entry
}

