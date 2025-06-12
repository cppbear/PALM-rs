// Answer 0

#[test]
fn test_append_value_with_existing_links() {
    let entry_idx = 0;
    let mut entry = Bucket {
        hash: 0,
        key: HeaderName::new("Test-Header").unwrap(),
        value: "InitialValue",
        links: Some(Links { next: 1, tail: 1 }),
    };

    let mut extra: Vec<ExtraValue<&str>> = Vec::new();
    extra.push(ExtraValue {
        value: "ExtraValue1",
        prev: Link::Entry(entry_idx),
        next: Link::Entry(entry_idx),
    });

    append_value(entry_idx, &mut entry, &mut extra, "ExtraValue2");

    assert_eq!(extra.len(), 2);
    assert_eq!(extra[1].value, "ExtraValue2");
    assert_eq!(entry.links.unwrap().tail, 1);
    assert_eq!(extra[0].next, Link::Extra(1));
}

#[test]
fn test_append_value_with_no_existing_links() {
    let entry_idx = 0;
    let mut entry = Bucket {
        hash: 0,
        key: HeaderName::new("Test-Header").unwrap(),
        value: "InitialValue",
        links: None,
    };

    let mut extra: Vec<ExtraValue<&str>> = Vec::new();
    append_value(entry_idx, &mut entry, &mut extra, "ExtraValue1");

    assert_eq!(extra.len(), 1);
    assert_eq!(extra[0].value, "ExtraValue1");
    assert_eq!(entry.links.unwrap().next, 0);
    assert_eq!(entry.links.unwrap().tail, 0);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_append_value_panic_on_extra_index() {
    let entry_idx = 0;
    let mut entry = Bucket {
        hash: 0,
        key: HeaderName::new("Test-Header").unwrap(),
        value: "InitialValue",
        links: Some(Links { next: 0, tail: 0 }),
    };

    let mut extra: Vec<ExtraValue<&str>> = Vec::new();
    extra.push(ExtraValue {
        value: "ExtraValue1",
        prev: Link::Entry(entry_idx),
        next: Link::Entry(entry_idx),
    });

    // This will panic because extra[0].next is being accessed without a valid next.
    append_value(entry_idx, &mut entry, &mut extra, "ExtraValue2");
}

