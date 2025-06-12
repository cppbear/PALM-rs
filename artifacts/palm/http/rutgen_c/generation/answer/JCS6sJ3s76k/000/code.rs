// Answer 0

#[test]
fn test_header_map_new() {
    struct TestHeaderValue;

    impl Default for TestHeaderValue {
        fn default() -> Self {
            TestHeaderValue
        }
    }

    let map: HeaderMap<TestHeaderValue> = HeaderMap::new();

    assert!(map.is_empty());
    assert_eq!(0, map.capacity());
}

