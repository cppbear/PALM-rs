// Answer 0

#[test]
fn test_remove_function() {
    struct TestEntry<K, V> {
        key: K,
        value: V,
    }
    
    struct TestMap<K, V> {
        entries: Vec<TestEntry<K, V>>,
    }
    
    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
        
        fn insert(&mut self, key: K, value: V) {
            self.entries.push(TestEntry { key, value });
        }
        
        fn swap_remove(&mut self, index: usize) -> V {
            let len = self.entries.len();
            let last_index = len - 1;
            self.entries.swap(index, last_index);
            self.entries.pop().unwrap().value
        }
        
        fn get_index(&self, key: &K) -> Option<usize>
        where
            K: PartialEq,
        {
            self.entries.iter().position(|entry| &entry.key == key)
        }
    }
    
    // Initialize test map and add some entries
    let mut map = TestMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);
    map.insert("key3", 30);
    
    // Remove an entry using its index
    let index_to_remove = map.get_index(&"key2").unwrap();
    let removed_value = map.swap_remove(index_to_remove);
    
    // Verify the removed value is correct
    assert_eq!(removed_value, 20);
    
    // Ensure the entry is removed from the map
    assert_eq!(map.entries.len(), 2);
    assert!(map.get_index(&"key2").is_none());
}

