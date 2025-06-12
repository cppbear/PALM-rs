// Answer 0

#[test]
fn test_try_insert_phase_two() {
    struct TestHeaderMap {
        map: HeaderMap<String>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: HeaderMap::with_capacity(16),
            }
        }

        fn insert_test_value(&mut self, key: HeaderName, value: String, probe: usize) -> usize {
            let hash = HashValue(0); // Simplified hash for testing
            let danger = false;
            self.map.try_insert_phase_two(key, value, hash, probe, danger).unwrap()
        }
    }

    let mut header_map = TestHeaderMap::new();
    
    // Prepare key and value
    let key1 = HeaderName { inner: Default::default() }; // Assume valid default
    let value1 = "value1".to_string();
    
    let index = header_map.insert_test_value(key1.clone(), value1.clone(), 0);
    assert_eq!(index, 0);
    assert_eq!(header_map.map.entries.len(), 1);
    
    // Test inserting more items
    let key2 = HeaderName { inner: Default::default() }; // Assume valid default
    let value2 = "value2".to_string();
    
    let index2 = header_map.insert_test_value(key2.clone(), value2.clone(), 1);
    assert_eq!(index2, 1);
    assert_eq!(header_map.map.entries.len(), 2);
} 

#[test]
#[should_panic]
fn test_try_insert_phase_two_over_capacity() {
    let mut header_map = HeaderMap::<String>::with_capacity(1);
    let key = HeaderName { inner: Default::default() }; // Assume valid default
    let value = "value".to_string();
    
    header_map.try_insert_phase_two(key.clone(), value.clone(), HashValue(0), 0, false).unwrap();
    // This should panic, as we are trying to exceed the capacity
    header_map.try_insert_phase_two(key, value, HashValue(0), 1, false).unwrap();
} 

#[test]
fn test_try_insert_phase_two_with_danger() {
    struct TestHeaderMap {
        map: HeaderMap<String>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: HeaderMap::with_capacity(16),
            }
        }

        fn insert_test_value(&mut self, key: HeaderName, value: String, probe: usize, danger: bool) -> usize {
            let hash = HashValue(0); // Simplified hash for testing
            self.map.try_insert_phase_two(key, value, hash, probe, danger).unwrap()
        }
    }

    let mut header_map = TestHeaderMap::new();
    
    let key1 = HeaderName { inner: Default::default() }; // Assume valid default
    let value1 = "value1".to_string();
    
    let index = header_map.insert_test_value(key1.clone(), value1.clone(), 0, false);
    assert_eq!(index, 0);
    assert_eq!(header_map.map.danger.is_yellow(), false);
    
    // Now inserting with danger
    let key2 = HeaderName { inner: Default::default() }; // Assume valid default
    let value2 = "value2".to_string();
    
    let _ = header_map.insert_test_value(key2.clone(), value2.clone(), 0, true);
    assert_eq!(header_map.map.danger.is_yellow(), true);
}

