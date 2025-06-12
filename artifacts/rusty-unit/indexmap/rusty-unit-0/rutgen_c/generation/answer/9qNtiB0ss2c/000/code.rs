// Answer 0

#[test]
fn test_last_entry_with_non_empty_map() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.entries.push((key, value));
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry {
                map: RefMut::from(&mut self.entries[index]),
                index,
            })
        }

        fn last_entry(&mut self) -> Option<IndexedEntry<'_, i32, String>> {
            self.get_index_entry(self.len().checked_sub(1)?)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());

    let last_entry = map.last_entry();
    assert!(last_entry.is_some());
    assert_eq!(last_entry.unwrap().map.1, "value2".to_string());
}

#[test]
fn test_last_entry_with_empty_map() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry {
                map: RefMut::from(&mut self.entries[index]),
                index,
            })
        }

        fn last_entry(&mut self) -> Option<IndexedEntry<'_, i32, String>> {
            self.get_index_entry(self.len().checked_sub(1)?)
        }
    }

    let mut map = TestMap::new();
    let last_entry = map.last_entry();
    assert!(last_entry.is_none());
}

