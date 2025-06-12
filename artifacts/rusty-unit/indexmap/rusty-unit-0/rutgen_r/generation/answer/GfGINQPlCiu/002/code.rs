// Answer 0

#[test]
fn test_get_index_entry_valid_index() {
    struct TestStruct {
        core: Vec<i32>,
    }

    impl TestStruct {
        fn len(&self) -> usize {
            self.core.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, usize, i32>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.core, index))
        }
    }

    let mut test_map = TestStruct { core: vec![10, 20, 30] };
    let index = 1;
    let entry = test_map.get_index_entry(index);
    assert!(entry.is_some());
}

#[test]
fn test_get_index_entry_out_of_bounds() {
    struct TestStruct {
        core: Vec<i32>,
    }

    impl TestStruct {
        fn len(&self) -> usize {
            self.core.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, usize, i32>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.core, index))
        }
    }

    let mut test_map = TestStruct { core: vec![10, 20, 30] };
    let index = 3; // Out of bounds
    let entry = test_map.get_index_entry(index);
    assert!(entry.is_none());
}

#[test]
fn test_get_index_entry_empty_map() {
    struct TestStruct {
        core: Vec<i32>,
    }

    impl TestStruct {
        fn len(&self) -> usize {
            self.core.len()
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, usize, i32>> {
            if index >= self.len() {
                return None;
            }
            Some(IndexedEntry::new(&mut self.core, index))
        }
    }

    let mut test_map = TestStruct { core: vec![] };
    let index = 0; // Index should be out of bounds
    let entry = test_map.get_index_entry(index);
    assert!(entry.is_none());
}

