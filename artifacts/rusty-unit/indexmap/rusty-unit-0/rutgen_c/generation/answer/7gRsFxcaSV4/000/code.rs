// Answer 0

#[test]
fn test_first_entry_empty() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, i32, String>> {
            self.get_index_entry(0)
        }

        pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.entries.len() {
                None
            } else {
                Some(IndexedEntry { map: &mut self.entries[index].1, index })
            }
        }
    }

    let mut map = TestMap { entries: Vec::new() };
    assert_eq!(map.first_entry(), None);
}

#[test]
fn test_first_entry_non_empty() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, i32, String>> {
            self.get_index_entry(0)
        }

        pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.entries.len() {
                None
            } else {
                Some(IndexedEntry { map: &mut self.entries[index].1, index })
            }
        }
    }

    let mut map = TestMap { entries: vec![(1, "one".to_string()), (2, "two".to_string())] };
    assert!(map.first_entry().is_some());
    if let Some(entry) = map.first_entry() {
        assert_eq!(entry.index, 0);
    }
}

#[test]
fn test_first_entry_boundary() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, i32, String>> {
            self.get_index_entry(0)
        }

        pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, String>> {
            if index >= self.entries.len() {
                None
            } else {
                Some(IndexedEntry { map: &mut self.entries[index].1, index })
            }
        }
    }

    let mut map = TestMap { entries: vec![(3, "three".to_string())] };
    assert!(map.first_entry().is_some());
    if let Some(entry) = map.first_entry() {
        assert_eq!(entry.index, 0);
    }
}

