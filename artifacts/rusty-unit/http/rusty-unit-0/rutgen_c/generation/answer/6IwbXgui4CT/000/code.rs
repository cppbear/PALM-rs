// Answer 0

#[test]
fn test_get_mut_existing_key() {
    struct TestHeaderName(String);
    
    impl AsHeaderName for TestHeaderName {
        fn find<K>(&self, map: &HeaderMap<K>) -> Option<(usize, usize)> {
            let key = self.0.to_lowercase();
            for (index, entry) in map.entries.iter().enumerate() {
                if entry.key.0.to_lowercase() == key {
                    return Some((index, index));
                }
            }
            None
        }
    }

    let mut header_map = HeaderMap::with_capacity(2);
    header_map.insert(TestHeaderName("host".to_string()), "hello".to_string());
    
    if let Some(value) = header_map.get_mut(TestHeaderName("host".to_string())) {
        value.push_str("-world");
    }

    assert_eq!(header_map.get(TestHeaderName("host".to_string())).unwrap(), &"hello-world");
}

#[test]
fn test_get_mut_non_existent_key() {
    struct TestHeaderName(String);
    
    impl AsHeaderName for TestHeaderName {
        fn find<K>(&self, map: &HeaderMap<K>) -> Option<(usize, usize)> {
            None
        }
    }

    let mut header_map = HeaderMap::with_capacity(2);
    header_map.insert(TestHeaderName("host".to_string()), "hello".to_string());

    let result = header_map.get_mut(TestHeaderName("content-type".to_string()));
    assert!(result.is_none());
}

#[test]
fn test_get_mut_multiple_keys() {
    struct TestHeaderName(String);
    
    impl AsHeaderName for TestHeaderName {
        fn find<K>(&self, map: &HeaderMap<K>) -> Option<(usize, usize)> {
            let key = self.0.to_lowercase();
            for (index, entry) in map.entries.iter().enumerate() {
                if entry.key.0.to_lowercase() == key {
                    return Some((index, index));
                }
            }
            None
        }
    }

    let mut header_map = HeaderMap::with_capacity(2);
    header_map.insert(TestHeaderName("item".to_string()), "value1".to_string());
    header_map.insert(TestHeaderName("item".to_string()), "value2".to_string());

    if let Some(value) = header_map.get_mut(TestHeaderName("item".to_string())) {
        value.push_str("-modified");
    }

    assert_eq!(header_map.get(TestHeaderName("item".to_string())).unwrap(), &"value1-modified");
}

