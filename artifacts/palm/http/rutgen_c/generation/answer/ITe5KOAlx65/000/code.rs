// Answer 0

#[test]
fn test_capacity_initial_empty() {
    struct TestHeaderValue;
    
    let map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(0);
    
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_after_single_insert() {
    struct TestHeaderValue;
    struct TestHeaderName(String);
    
    impl AsHeaderName for TestHeaderName {
        fn as_header_name(&self) -> &HeaderName {
            &HeaderName::from(self.0.as_str())
        }
    }

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(6);
    
    map.insert(TestHeaderName("Host".to_string()), TestHeaderValue);
    
    assert_eq!(map.capacity(), 6);
}

#[test]
fn test_capacity_after_multiple_inserts() {
    struct TestHeaderValue;
    struct TestHeaderName(String);
    
    impl AsHeaderName for TestHeaderName {
        fn as_header_name(&self) -> &HeaderName {
            &HeaderName::from(self.0.as_str())
        }
    }

    let mut map: HeaderMap<TestHeaderValue> = HeaderMap::with_capacity(10);
    
    map.insert(TestHeaderName("Host".to_string()), TestHeaderValue);
    map.insert(TestHeaderName("User-Agent".to_string()), TestHeaderValue);
    
    assert_eq!(map.capacity(), 10);
}

