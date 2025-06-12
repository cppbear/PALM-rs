// Answer 0

#[test]
fn test_shift_remove_entry_valid() {
    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<(i32, String)>,
    }
    
    impl TestMap {
        fn new() -> Self {
            Self {
                indices: vec![],
                entries: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
            }
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.entries.len() {
                let removed_entry = self.entries.remove(index);
                self.indices.retain(|&i| i != index);
                for i in index..self.indices.len() {
                    self.indices[i] -= 1;
                }
                Some(removed_entry)
            } else {
                None
            }
        }
    }

    let mut map_core = TestMap::new();
    let index = 1;

    let indexed_entry = IndexedEntry::new(&mut map_core, index);
    let (key, value) = indexed_entry.shift_remove_entry();

    assert_eq!(key, 2);
    assert_eq!(value, "two");
    assert_eq!(map_core.entries.len(), 2);
}

#[test]
#[should_panic]
fn test_shift_remove_entry_out_of_bounds() {
    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                indices: vec![],
                entries: vec![(1, "one".to_string()), (2, "two".to_string())],
            }
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.entries.len() {
                let removed_entry = self.entries.remove(index);
                self.indices.retain(|&i| i != index);
                for i in index..self.indices.len() {
                    self.indices[i] -= 1;
                }
                Some(removed_entry)
            } else {
                None
            }
        }
    }

    let mut map_core = TestMap::new();
    let index = 5; // Out of bounds

    let indexed_entry = IndexedEntry::new(&mut map_core, index);
    indexed_entry.shift_remove_entry();
}

#[test]
fn test_shift_remove_entry_empty_map() {
    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                indices: vec![],
                entries: vec![],
            }
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.entries.len() {
                let removed_entry = self.entries.remove(index);
                self.indices.retain(|&i| i != index);
                for i in index..self.indices.len() {
                    self.indices[i] -= 1;
                }
                Some(removed_entry)
            } else {
                None
            }
        }
    }

    let mut map_core = TestMap::new();
    let index = 0; // Index does not exist

    let indexed_entry = IndexedEntry::new(&mut map_core, index);
    let removed_entry = indexed_entry.shift_remove_entry();
    assert!(removed_entry.is_none());
}

