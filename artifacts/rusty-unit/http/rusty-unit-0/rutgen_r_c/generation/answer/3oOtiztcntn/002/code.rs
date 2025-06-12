// Answer 0

#[test]
fn test_append_value_none_links() {
    let entry_idx = 0;
    let mut entry = Bucket {
        hash: 0, // Assuming a hash value of 0 for testing
        key: HeaderName::from_static("test-header"), // Assuming a constructor for HeaderName
        value: "initial_value", // Placeholder type, change as needed for actual type
        links: None,
    };
    let mut extra = Vec::new();
    let new_value = "new_value"; // Placeholder type, change as needed for actual type

    append_value(entry_idx, &mut entry, &mut extra, new_value);

    assert_eq!(extra.len(), 1);
    assert_eq!(extra[0].value, new_value);
    assert_eq!(entry.links.as_ref().unwrap().tail, 0);
    assert_eq!(entry.links.as_ref().unwrap().next, 0);
    assert_eq!(extra[0].prev, Link::Entry(entry_idx));
    assert_eq!(extra[0].next, Link::Entry(entry_idx));
}

#[test]
fn test_append_value_some_links() {
    let entry_idx = 1;
    let mut entry = Bucket {
        hash: 0, // Assuming a hash value of 0 for testing
        key: HeaderName::from_static("test-header-2"), // Assuming a constructor for HeaderName
        value: "initial_value", // Placeholder type, change as needed for actual type
        links: Some(Links { next: 0, tail: 0 }),
    };
    let mut extra = vec![ExtraValue {
        value: "existing_value", // Placeholder type, change as needed for actual type
        prev: Link::Entry(entry_idx),
        next: Link::Entry(entry_idx),
    }];
    let new_value = "new_value"; // Placeholder type, change as needed for actual type

    append_value(entry_idx, &mut entry, &mut extra, new_value);

    assert_eq!(extra.len(), 2);
    assert_eq!(extra[1].value, new_value);
    assert_eq!(entry.links.as_ref().unwrap().tail, 1);
    assert_eq!(entry.links.as_ref().unwrap().next, 0);
    assert_eq!(extra[1].prev, Link::Extra(entry.links.as_ref().unwrap().tail));
    assert_eq!(extra[1].next, Link::Entry(entry_idx));
    assert_eq!(extra[0].next, Link::Extra(1));
}

