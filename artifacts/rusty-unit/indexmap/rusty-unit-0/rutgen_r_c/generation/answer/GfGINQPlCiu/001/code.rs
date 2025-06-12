// Answer 0

#[test]
fn test_get_index_entry_out_of_bounds() {
    struct TestMap {
        core: IndexMapCore<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                }
            }
        }
        
        fn len(&self) -> usize {
            self.core.entries.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.core, index))
        }
    }

    let mut map = TestMap::new();

    // Attempt to get an entry with an index equal to the current length of the map (which is 0).
    assert_eq!(map.get_index_entry(0), None);
}

#[test]
fn test_get_index_entry_exactly_out_of_bounds() {
    struct TestMap {
        core: IndexMapCore<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                }
            }
        }
        
        fn len(&self) -> usize {
            self.core.entries.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.core, index))
        }
    }

    let mut map = TestMap::new();
    // Populate the map with data by simulating adding entries.
    // Assume we add two entries (index 0 and 1) for this test.
    map.core.entries.push(("key1".to_string(), "value1".to_string()));
    map.core.entries.push(("key2".to_string(), "value2".to_string()));

    // Attempt to get an entry with an index equal to the current length of the map (which is 2).
    assert_eq!(map.get_index_entry(2), None);
}

