// Answer 0

#[test]
fn test_insert_mult_with_existing_values() {
    // Define necessary structs for the test
    struct FakeHeaderValue(String);
    impl From<&str> for FakeHeaderValue {
        fn from(value: &str) -> Self {
            FakeHeaderValue(value.to_string())
        }
    }
    
    // Initialize the HeaderMap
    let mut map = HeaderMap::<FakeHeaderValue>::with_capacity(10);

    // Insert initial values
    map.insert(HeaderName::from_static("host"), FakeHeaderValue::from("world"));
    map.append(HeaderName::from_static("host"), FakeHeaderValue::from("world2"));

    // Test inserting multiple values
    if let Entry::Occupied(mut entry) = map.entry(HeaderName::from_static("host")) {
        let mut drain = entry.insert_mult(FakeHeaderValue::from("earth"));
        
        // Verify the previous values are returned correctly
        assert_eq!("world", drain.next().unwrap().0);
        assert_eq!("world2", drain.next().unwrap().0);
        assert!(drain.next().is_none());
    }

    // Verify the current value for "host"
    assert_eq!("earth", map.get(HeaderName::from_static("host")).unwrap().0);
}

#[test]
fn test_insert_mult_empty_map() {
    // Initialize an empty HeaderMap
    let mut map = HeaderMap::<FakeHeaderValue>::with_capacity(10);

    // Test inserting a value
    if let Entry::Vacant(entry) = map.entry(HeaderName::from_static("new-key")) {
        let mut drain = entry.insert_mult(FakeHeaderValue::from("new-value"));
        
        // Verify that no previous values exist
        assert!(drain.next().is_none());
    }

    // Verify the current value for "new-key"
    assert_eq!("new-value", map.get(HeaderName::from_static("new-key")).unwrap().0);
}

#[test]
#[should_panic]
fn test_insert_mult_exceeds_capacity() {
    // Define necessary structs for the test
    struct LimitedHeaderValue(String);
    impl From<&str> for LimitedHeaderValue {
        fn from(value: &str) -> Self {
            LimitedHeaderValue(value.to_string())
        }
    }
    
    // Initialize the HeaderMap with a small capacity
    let mut map = HeaderMap::<LimitedHeaderValue>::try_with_capacity(1).unwrap();

    // Insert a value
    map.insert(HeaderName::from_static("host"), LimitedHeaderValue::from("initial"));

    // This should panic because inserting exceeds the capacity
    if let Entry::Occupied(mut entry) = map.entry(HeaderName::from_static("host")) {
        entry.insert_mult(LimitedHeaderValue::from("second"));
    }
}

