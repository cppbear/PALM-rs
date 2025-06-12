// Answer 0

#[test]
fn test_insert_occupied() {
    struct TestValue {
        data: String,
    }

    let mut header_map = HeaderMap {
        mask: 0,
        indices: Box::new([Pos { index: 0, hash: 0 }]),
        entries: vec![Bucket {
            hash: 0,
            key: HeaderName::from_static("test-key"),
            value: TestValue { data: "old_value".to_string() },
            links: None,
        }],
        extra_values: vec![],
        danger: Danger::Green,
    };

    let new_value = TestValue { data: "new_value".to_string() };
    let replaced_value = header_map.insert_occupied(0, new_value);

    assert_eq!(replaced_value.data, "old_value");
    assert_eq!(header_map.entries[0].value.data, "new_value");
}

#[test]
fn test_insert_occupied_with_links() {
    struct TestValue {
        data: String,
    }

    let mut header_map = HeaderMap {
        mask: 0,
        indices: Box::new([Pos { index: 0, hash: 0 }]),
        entries: vec![Bucket {
            hash: 0,
            key: HeaderName::from_static("test-key"),
            value: TestValue { data: "old_value".to_string() },
            links: Some(Links { next: 1, tail: 1 }),
        }],
        extra_values: vec![ExtraValue {
            value: TestValue { data: "extra_value".to_string() },
            prev: Link::None,
            next: Link::Extra(0),
        }],
        danger: Danger::Green,
    };

    let new_value = TestValue { data: "new_value".to_string() };
    let replaced_value = header_map.insert_occupied(0, new_value);

    assert_eq!(replaced_value.data, "old_value");
    assert_eq!(header_map.entries[0].value.data, "new_value");
    assert!(header_map.extra_values.is_empty());
}

