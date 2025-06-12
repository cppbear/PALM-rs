// Answer 0

#[test]
fn test_push_entry() {
    struct HashValue(u64);
    
    struct Bucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }
    
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }
    
    impl<K, V> TestMap<K, V> {
        fn new(capacity: usize) -> Self {
            TestMap {
                entries: Vec::with_capacity(capacity),
            }
        }
        
        fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
            if self.entries.len() == self.entries.capacity() {
                self.reserve_entries(1);
            }
            self.entries.push(Bucket { hash, key, value });
        }
        
        fn reserve_entries(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }
    }

    let mut test_map: TestMap<String, String> = TestMap::new(2);
    test_map.push_entry(HashValue(1), "key1".to_string(), "value1".to_string());
    test_map.push_entry(HashValue(2), "key2".to_string(), "value2".to_string());

    assert_eq!(test_map.entries.len(), 2);
    assert_eq!(test_map.entries[0].key, "key1");
    assert_eq!(test_map.entries[0].value, "value1");
    assert_eq!(test_map.entries[1].key, "key2");
    assert_eq!(test_map.entries[1].value, "value2");
}

#[test]
fn test_push_entry_growth() {
    struct HashValue(u64);
    
    struct Bucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }
    
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }
    
    impl<K, V> TestMap<K, V> {
        fn new(capacity: usize) -> Self {
            TestMap {
                entries: Vec::with_capacity(capacity),
            }
        }
        
        fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
            if self.entries.len() == self.entries.capacity() {
                self.reserve_entries(1);
            }
            self.entries.push(Bucket { hash, key, value });
        }
        
        fn reserve_entries(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }
    }

    let mut test_map: TestMap<String, String> = TestMap::new(1);
    test_map.push_entry(HashValue(1), "key1".to_string(), "value1".to_string());
    
    assert_eq!(test_map.entries.len(), 1);

    // This will trigger the growth of the underlying vector
    test_map.push_entry(HashValue(2), "key2".to_string(), "value2".to_string());
    
    assert_eq!(test_map.entries.len(), 2);
    assert_eq!(test_map.entries[0].key, "key1");
    assert_eq!(test_map.entries[1].key, "key2");
}

