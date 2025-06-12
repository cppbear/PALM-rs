// Answer 0

#[test]
fn test_append_value_with_existing_links() {
    let entry_idx = 0;
    let mut extra: Vec<ExtraValue<i32>> = Vec::new();
    let mut entry = Bucket {
        hash: 0, // assuming a value for hash, since it's not defined in the context
        key: HeaderName::new("Test-Header"), // Create a valid HeaderName
        value: 42,
        links: Some(Links { next: 1, tail: 0 }),
    };

    extra.push(ExtraValue {
        value: 2,
        prev: Link::Entry(entry_idx),
        next: Link::Entry(entry_idx),
    });

    append_value(entry_idx, &mut entry, &mut extra, 99);

    assert_eq!(extra.len(), 2);
    assert_eq!(entry.links.as_ref().unwrap().tail, 1);
    assert_eq!(extra[1].value, 99);
    if let Link::Extra(prev_link) = extra[1].prev {
        assert_eq!(prev_link, 0);
    }
    assert_eq!(extra[0].next, Link::Extra(1));
}

#[test]
fn test_append_value_with_no_existing_links() {
    let entry_idx = 0;
    let mut extra: Vec<ExtraValue<i32>> = Vec::new();
    let mut entry = Bucket {
        hash: 0, // assuming a value for hash
        key: HeaderName::new("Another-Header"), // Create a valid HeaderName
        value: 42,
        links: None,
    };

    append_value(entry_idx, &mut entry, &mut extra, 55);

    assert_eq!(extra.len(), 1);
    assert_eq!(entry.links.as_ref().unwrap().tail, 0);
    assert_eq!(entry.links.as_ref().unwrap().next, 0);
    assert_eq!(extra[0].value, 55);
    assert_eq!(extra[0].prev, Link::Entry(entry_idx));
    assert_eq!(extra[0].next, Link::Entry(entry_idx));
}

