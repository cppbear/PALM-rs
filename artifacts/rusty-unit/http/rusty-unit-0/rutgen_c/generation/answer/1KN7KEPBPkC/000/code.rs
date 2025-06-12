// Answer 0

#[test]
fn test_header_map_debug_display_empty() {
    let map: HeaderMap<i32> = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: Vec::new(),
        extra_values: Vec::new(),
        danger: Danger::Green,
    };
    let result = format!("{:?}", map);
    assert_eq!(result, "{}");
}

#[test]
fn test_header_map_debug_display_with_entries() {
    #[derive(Debug, PartialEq)]
    struct TestValue {
        id: i32,
    }

    let key = HeaderName::from_static("test-key");
    let value = TestValue { id: 42 };

    let mut map: HeaderMap<TestValue> = HeaderMap {
        mask: 0,
        indices: Box::new([Pos { index: 0, hash: 0 }]),
        entries: vec![Bucket {
            hash: 0,
            key,
            value,
            links: None,
        }],
        extra_values: Vec::new(),
        danger: Danger::Green,
    };
    
    let result = format!("{:?}", map);
    assert!(result.contains("test-key"));
    assert!(result.contains("42"));
}

#[test]
fn test_header_map_debug_display_multiple_entries() {
    #[derive(Debug, PartialEq)]
    struct TestValue {
        id: i32,
    }

    let key1 = HeaderName::from_static("key1");
    let value1 = TestValue { id: 1 };
    let key2 = HeaderName::from_static("key2");
    let value2 = TestValue { id: 2 };

    let mut map: HeaderMap<TestValue> = HeaderMap {
        mask: 0,
        indices: Box::new([Pos { index: 0, hash: 0 }, Pos { index: 1, hash: 1 }]),
        entries: vec![
            Bucket {
                hash: 0,
                key: key1,
                value: value1,
                links: None,
            },
            Bucket {
                hash: 1,
                key: key2,
                value: value2,
                links: None,
            },
        ],
        extra_values: Vec::new(),
        danger: Danger::Green,
    };

    let result = format!("{:?}", map);
    assert!(result.contains("key1"));
    assert!(result.contains("key2"));
    assert!(result.contains("1"));
    assert!(result.contains("2"));
}

