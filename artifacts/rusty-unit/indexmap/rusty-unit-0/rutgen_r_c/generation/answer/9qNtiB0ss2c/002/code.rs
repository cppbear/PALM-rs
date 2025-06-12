// Answer 0

#[test]
fn test_last_entry_with_non_empty_map() {
    struct TestMap {
        data: Vec<(i32, i32)>,
        length: usize,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                data: Vec::new(),
                length: 0,
            }
        }
        
        fn push(&mut self, key: i32, value: i32) {
            self.data.push((key, value));
            self.length += 1;
        }
        
        fn len(&self) -> usize {
            self.length
        }
        
        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index < self.len() {
                Some(IndexedEntry { map: &mut self.data[index].1, index })
            } else {
                None
            }
        }

        fn last_entry(&mut self) -> Option<IndexedEntry<'_, i32, i32>> {
            self.get_index_entry(self.len().checked_sub(1)?)
        }
    }

    let mut test_map = TestMap::new();
    test_map.push(1, 10);
    test_map.push(2, 20);
    test_map.push(3, 30);

    let entry = test_map.last_entry();
    assert!(entry.is_some());
    if let Some(IndexedEntry { map, index }) = entry {
        assert_eq!(*map, 30);
        assert_eq!(index, 2);
    }
}

#[test]
fn test_last_entry_with_empty_map() {
    struct TestMap {
        data: Vec<(i32, i32)>,
        length: usize,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                data: Vec::new(),
                length: 0,
            }
        }

        fn len(&self) -> usize {
            self.length
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, i32, i32>> {
            if index < self.len() {
                Some(IndexedEntry { map: &mut self.data[index].1, index })
            } else {
                None
            }
        }

        fn last_entry(&mut self) -> Option<IndexedEntry<'_, i32, i32>> {
            self.get_index_entry(self.len().checked_sub(1)?)
        }
    }

    let mut test_map = TestMap::new();
    
    let entry = test_map.last_entry();
    assert!(entry.is_none());
}

