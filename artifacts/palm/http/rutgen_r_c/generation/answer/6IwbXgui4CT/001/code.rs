// Answer 0

#[test]
fn test_get_mut_existing_key() {
    struct TestHeaderName(String);
    impl AsHeaderName for TestHeaderName {
        fn find(&self, header_map: &HeaderMap<String>) -> Option<(usize, usize)> {
            for (index, entry) in header_map.entries.iter().enumerate() {
                if &entry.key == &self.0 {
                    return Some((index, index)); // Simulate matching on key
                }
            }
            None
        }
    }

    let mut map = HeaderMap::with_capacity(10);
    let key = "host".to_string();
    map.insert(TestHeaderName(key.clone()), "value".to_string());

    if let Some(value) = map.get_mut(TestHeaderName(key)) {
        value.push_str("-updated");
    }

    assert_eq!(map.get(TestHeaderName(key)).unwrap(), &"value-updated".to_string());
}

#[test]
fn test_get_mut_non_existing_key() {
    struct TestHeaderName(String);
    impl AsHeaderName for TestHeaderName {
        fn find(&self, header_map: &HeaderMap<String>) -> Option<(usize, usize)> {
            None // Simulate no match
        }
    }

    let mut map = HeaderMap::with_capacity(10);
    let key = "host".to_string();
    map.insert(TestHeaderName(key.clone()), "value".to_string());

    assert!(map.get_mut(TestHeaderName("non-existent".to_string())).is_none());
}

