// Answer 0

#[test]
fn test_shift_remove_index_none() {
    struct TestMap<K, V> {
        entries: Vec<Option<(K, V)>>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            match self.entries.get(index) {
                Some(entry) => {
                    self.indices.remove(index); // Simulate erase_index
                    Some(entry.clone().unwrap()) // Simulate shift_remove_finish
                }
                None => None,
            }
        }
    }

    let mut test_map: TestMap<i32, &str> = TestMap::new();
    
    // Call shift_remove_index with an index that is out of bounds (0 for an empty map)
    let result = test_map.shift_remove_index(0);
    
    // Assert that the result is None
    assert_eq!(result, None);
}

