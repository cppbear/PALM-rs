// Answer 0

#[test]
fn test_insert_mult_with_single_value() {
    struct TestHeaderValue;

    impl From<&str> for TestHeaderValue {
        fn from(_: &str) -> Self {
            TestHeaderValue
        }
    }

    let mut map = http::HeaderMap::with_capacity(5);
    let header_name = http::HeaderName::from_static("host");

    map.insert(header_name.clone(), TestHeaderValue::from("world"));
    
    if let http::Entry::Occupied(mut entry) = map.entry(header_name) {
        let mut prev = entry.insert_mult(TestHeaderValue::from("earth"));
        assert_eq!(prev.next().is_none()); // No previous value should be returned
    }
    
    assert_eq!(map.get(header_name).unwrap(), &TestHeaderValue::from("earth"));
}

#[test]
fn test_insert_mult_with_multiple_values() {
    struct TestHeaderValue;

    impl From<&str> for TestHeaderValue {
        fn from(_: &str) -> Self {
            TestHeaderValue
        }
    }
    
    let mut map = http::HeaderMap::with_capacity(5);
    let header_name = http::HeaderName::from_static("host");

    map.insert(header_name.clone(), TestHeaderValue::from("world"));
    map.append(header_name.clone(), TestHeaderValue::from("world2"));

    if let http::Entry::Occupied(mut entry) = map.entry(header_name) {
        let mut prev = entry.insert_mult(TestHeaderValue::from("earth"));
        assert_eq!(prev.next().unwrap(), TestHeaderValue::from("world")); // First previous value
        assert_eq!(prev.next().unwrap(), TestHeaderValue::from("world2")); // Second previous value
        assert!(prev.next().is_none()); // No more previous values
    }
    
    assert_eq!(map.get(header_name).unwrap(), &TestHeaderValue::from("earth"));
}

#[test]
#[should_panic]
fn test_insert_mult_on_empty_entry() {
    struct TestHeaderValue;

    impl From<&str> for TestHeaderValue {
        fn from(_: &str) -> Self {
            TestHeaderValue
        }
    }

    let mut map = http::HeaderMap::with_capacity(5);
    let header_name = http::HeaderName::from_static("host");

    if let http::Entry::Occupied(mut entry) = map.entry(header_name) {
        entry.insert_mult(TestHeaderValue::from("earth")); // Should panic because entry is empty
    }
}

#[test]
fn test_insert_mult_after_removal() {
    struct TestHeaderValue;

    impl From<&str> for TestHeaderValue {
        fn from(_: &str) -> Self {
            TestHeaderValue
        }
    }

    let mut map = http::HeaderMap::with_capacity(5);
    let header_name = http::HeaderName::from_static("host");

    map.insert(header_name.clone(), TestHeaderValue::from("world"));

    map.remove(header_name.clone()); // Remove the entry

    // Now the map should be empty, trying to insert_mult should not panic
    if let http::Entry::Vacant(entry) = map.entry(header_name) {
        entry.insert_mult(TestHeaderValue::from("earth")); // Safe operation
    }
    
    assert_eq!(map.get(header_name).unwrap(), &TestHeaderValue::from("earth"));
}

