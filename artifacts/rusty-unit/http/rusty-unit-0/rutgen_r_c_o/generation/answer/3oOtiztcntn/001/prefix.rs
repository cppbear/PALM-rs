// Answer 0

#[test]
fn test_append_value_with_existing_links() {
    let entry_idx = 5;
    let mut entry = Bucket {
        hash: 0, // Placeholder; hash value not used in the test
        key: HeaderName::from_str("Example-Key").unwrap(),
        value: 10, // Placeholder; actual type not relevant for the test
        links: Some(Links { next: 2, tail: 3 }),
    };
    let mut extra: Vec<ExtraValue<i32>> = vec![
        ExtraValue {
            value: 20,
            prev: Link::Entry(1),
            next: Link::Entry(2),
        },
        ExtraValue {
            value: 30,
            prev: Link::Entry(0),
            next: Link::Entry(1),
        },
        ExtraValue {
            value: 40,
            prev: Link::Entry(2),
            next: Link::Entry(3),
        },
    ];
    let value = 50;

    append_value(entry_idx, &mut entry, &mut extra, value);
}

#[test]
fn test_append_value_with_no_links() {
    let entry_idx = 2;
    let mut entry = Bucket {
        hash: 0, 
        key: HeaderName::from_str("Another-Key").unwrap(),
        value: 15, 
        links: None,
    };
    let mut extra: Vec<ExtraValue<i32>> = vec![];
    let value = 25;

    append_value(entry_idx, &mut entry, &mut extra, value);
}

#[test]
fn test_append_value_with_empty_extra() {
    let entry_idx = 0;
    let mut entry = Bucket {
        hash: 0, 
        key: HeaderName::from_str("Key-Empty").unwrap(),
        value: 5, 
        links: Some(Links { next: 0, tail: 0 }),
    };
    let mut extra: Vec<ExtraValue<i32>> = vec![];
    let value = 45;

    append_value(entry_idx, &mut entry, &mut extra, value);
}

#[test]
fn test_append_value_with_max_extra_length() {
    let entry_idx = 32767;
    let mut entry = Bucket {
        hash: 0, 
        key: HeaderName::from_str("Max-Extra-Key").unwrap(),
        value: 100, 
        links: Some(Links { next: 32766, tail: 32766 }),
    };
    let mut extra: Vec<ExtraValue<i32>> = (0..32767).map(|i| ExtraValue {
        value: i,
        prev: Link::Extra(i.checked_sub(1).unwrap_or(0)),
        next: Link::Entry(i),
    }).collect();
    let value = 200;

    append_value(entry_idx, &mut entry, &mut extra, value);
}

