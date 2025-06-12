// Answer 0

#[test]
fn test_index_existing_key() {
    struct TestKey;
    impl AsHeaderName for TestKey {
        fn as_str(&self) -> &str {
            "valid_key"
        }
    }

    let mut header_map = HeaderMap::<HeaderValue> {
        mask: 0,
        indices: Box::new([]),
        entries: vec![Bucket {
            hash: 0,
            key: HeaderName::from("valid_key").unwrap(),
            value: HeaderValue::from("value"),
            links: None,
        }],
        extra_values: vec![],
        danger: Danger::Green,
    };

    let value = header_map[TestKey];
    assert_eq!(value, &HeaderValue::from("value"));
}

#[test]
#[should_panic(expected = "no entry found for key \"invalid_key\"")]
fn test_index_non_existing_key() {
    struct TestKey;
    impl AsHeaderName for TestKey {
        fn as_str(&self) -> &str {
            "invalid_key"
        }
    }

    let header_map = HeaderMap::<HeaderValue> {
        mask: 0,
        indices: Box::new([]),
        entries: vec![],
        extra_values: vec![],
        danger: Danger::Green,
    };

    let _value = header_map[TestKey];
}

