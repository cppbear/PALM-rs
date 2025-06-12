// Answer 0

#[test]
fn test_get_index_entry_valid_index() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![(1, 10), (2, 20), (3, 30)] }
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }
        
        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.entries, index))
        }
    }

    let mut map = TestMap::new();
    let entry = map.get_index_entry(1);
    assert!(entry.is_some());
    let indexed_entry = entry.unwrap();
    assert_eq!(indexed_entry.index(), 1);
    assert_eq!(indexed_entry.get().1, &20);
}

#[test]
fn test_get_index_entry_invalid_index() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![(1, 10), (2, 20), (3, 30)] }
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }
        
        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.entries, index))
        }
    }

    let mut map = TestMap::new();
    let entry = map.get_index_entry(3); // out of bounds
    assert!(entry.is_none());
}

#[test]
fn test_get_index_entry_zero_index() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![(1, 10), (2, 20), (3, 30)] }
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }
        
        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.entries, index))
        }
    }

    let mut map = TestMap::new();
    let entry = map.get_index_entry(0);
    assert!(entry.is_some());
    let indexed_entry = entry.unwrap();
    assert_eq!(indexed_entry.index(), 0);
    assert_eq!(indexed_entry.get().1, &10);
}

