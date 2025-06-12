// Answer 0

#[test]
fn test_last_entry_empty_map() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index >= self.len() {
                None
            } else {
                Some(IndexedEntry {
                    map: &mut self.entries[index].1, // Simplified access for test purposes
                    index,
                })
            }
        }

        pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, i32, i32>> {
            self.get_index_entry(self.len().checked_sub(1)?)
        }
    }

    let mut map = TestMap::new();
    let result = map.last_entry();
    assert!(result.is_none());
}

#[test]
fn test_last_entry_single_element() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn push(&mut self, key: i32, value: i32) {
            self.entries.push((key, value));
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index >= self.len() {
                None
            } else {
                Some(IndexedEntry {
                    map: &mut self.entries[index].1,
                    index,
                })
            }
        }

        pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, i32, i32>> {
            self.get_index_entry(self.len().checked_sub(1)?)
        }
    }

    let mut map = TestMap::new();
    map.push(1, 42);

    let result = map.last_entry();
    assert!(result.is_some());
}

#[test]
fn test_last_entry_multiple_elements() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn push(&mut self, key: i32, value: i32) {
            self.entries.push((key, value));
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index >= self.len() {
                None
            } else {
                Some(IndexedEntry {
                    map: &mut self.entries[index].1,
                    index,
                })
            }
        }

        pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, i32, i32>> {
            self.get_index_entry(self.len().checked_sub(1)?)
        }
    }

    let mut map = TestMap::new();
    map.push(1, 42);
    map.push(2, 99);
    map.push(3, 100);

    let result = map.last_entry();
    assert!(result.is_some());
}

