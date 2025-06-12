// Answer 0

#[test]
fn test_remove_extra_value_entry_entry() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10]))); // Assume size of 10 for demonstration
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    // Setup initial state
    raw_links.0[0] = Some(Links { next: 0, tail: 0 });
    raw_links.0[1] = Some(Links { next: 1, tail: 1 });

    let idx = 1; // Index to remove
    let extra = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(extra.value, 2);
    assert_eq!(extra.prev, Link::Entry(1));
    assert_eq!(extra.next, Link::Entry(1));
}

#[test]
fn test_remove_extra_value_entry_extra() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 3, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 4, prev: Link::Extra(0), next: Link::Entry(1) },
    ];

    raw_links.0[0] = Some(Links { next: 0, tail: 0 });

    let idx = 1;
    let extra = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(extra.value, 4);
    assert_eq!(extra.prev, Link::Extra(0));
    assert_eq!(extra.next, Link::Entry(1));
}

#[test]
fn test_remove_extra_value_extra_entry() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 5, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 6, prev: Link::Entry(1), next: Link::Extra(0) },
    ];

    raw_links.0[1] = Some(Links { next: 1, tail: 1 });

    let idx = 0;
    let extra = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(extra.value, 5);
    assert_eq!(extra.prev, Link::Extra(0));
    assert_eq!(extra.next, Link::Entry(1));
}

#[test]
fn test_remove_extra_value_extra_extra() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 7, prev: Link::Extra(1), next: Link::Extra(1) },
        ExtraValue { value: 8, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 9, prev: Link::Extra(2), next: Link::Extra(0) },
    ];

    let idx = 1;
    let extra = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(extra.value, 8);
    assert_eq!(extra.prev, Link::Extra(0));
    assert_eq!(extra.next, Link::Extra(2));
}

