// Answer 0

#[test]
fn test_insert_occupied_with_valid_index() {
    struct TestValue {
        data: String,
    }

    let mut header_map = HeaderMap {
        mask: 0,
        indices: vec![Pos { index: 0, hash: 0 }].into_boxed_slice(),
        entries: vec![
            Bucket {
                hash: 123,
                key: HeaderName::from("Header-Key-1").unwrap(),
                value: TestValue { data: "Initial".to_string() },
                links: Some(Links { next: 1, tail: 1 }),
            },
            Bucket {
                hash: 456,
                key: HeaderName::from("Header-Key-2").unwrap(),
                value: TestValue { data: "To Be Removed".to_string() },
                links: None,
            },
        ],
        extra_values: vec![],
        danger: Danger::Green,
    };

    let new_value = TestValue { data: "Updated".to_string() };
    let old_value = header_map.insert_occupied(0, new_value);

    assert_eq!(old_value.data, "Initial");
    assert_eq!(header_map.entries[0].value.data, "Updated");
}

#[test]
#[should_panic]
fn test_insert_occupied_with_invalid_index() {
    struct TestValue {
        data: String,
    }

    let mut header_map = HeaderMap {
        mask: 0,
        indices: vec![Pos { index: 0, hash: 0 }].into_boxed_slice(),
        entries: vec![
            Bucket {
                hash: 123,
                key: HeaderName::from("Header-Key-1").unwrap(),
                value: TestValue { data: "Initial".to_string() },
                links: Some(Links { next: 1, tail: 1 }),
            },
        ],
        extra_values: vec![],
        danger: Danger::Green,
    };

    let new_value = TestValue { data: "Updated".to_string() };
    header_map.insert_occupied(1, new_value); // This should panic
}

