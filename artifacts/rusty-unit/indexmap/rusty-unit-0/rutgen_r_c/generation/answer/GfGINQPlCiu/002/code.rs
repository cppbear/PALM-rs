// Answer 0

#[test]
fn test_get_index_entry_valid_index() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: vec![(1, String::from("one")), (2, String::from("two"))],
            }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.entries, index))
        }
    }

    let mut map = TestMap::new();

    assert!(map.get_index_entry(0).is_some());
    assert!(map.get_index_entry(1).is_some());
}

#[test]
fn test_get_index_entry_out_of_bounds() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: vec![(1, String::from("one")), (2, String::from("two"))],
            }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.entries, index))
        }
    }

    let mut map = TestMap::new();

    assert!(map.get_index_entry(2).is_none());
}

