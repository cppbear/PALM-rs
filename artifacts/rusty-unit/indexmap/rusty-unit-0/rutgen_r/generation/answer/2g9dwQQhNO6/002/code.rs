// Answer 0

#[test]
fn test_swap_remove_index_none() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }
    
    impl<K, V> TestMap<K, V> {
        fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            match self.entries.get(index) {
                Some(entry) => {
                    // dummy erase_index function for this test
                    self.entries.swap_remove(index); 
                    Some(entry.clone())
                }
                None => None,
            }
        }
    }

    let mut map: TestMap<i32, &str> = TestMap {
        entries: Vec::new(),
    };

    // Test swapping an index that does not exist (out of bounds)
    let result = map.swap_remove_index(0);
    assert_eq!(result, None);
}

