// Answer 0

#[test]
fn test_remove_extra_value_with_entry_link() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 200, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 300, prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    raw_links.0[0] = Some(Bucket { next: 1, tail: 2 });

    let removed = remove_extra_value(raw_links, &mut extra_values, 1);
    
    assert_eq!(removed.value, 200);
    assert_eq!(extra_values.len(), 2);
    assert_ne!(extra_values[0].next, Link::Extra(1));
}

#[test]
fn test_remove_extra_value_with_extra_link() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 200, prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { value: 300, prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    raw_links.0[1] = Some(Bucket { next: 2, tail: 2 });

    let removed = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(removed.value, 100);
    assert_eq!(extra_values.len(), 2);
    assert_ne!(extra_values[1].prev, Link::Extra(0));
}

#[test]
fn test_remove_extra_value_removes_middle_entry() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 200, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 300, prev: Link::Extra(0), next: Link::Extra(2) },
    ];

    raw_links.0[0] = Some(Bucket { next: 1, tail: 2 });

    let removed = remove_extra_value(raw_links, &mut extra_values, 1);
    
    assert_eq!(removed.value, 200);
    assert_eq!(extra_values.len(), 2);
    assert_ne!(extra_values[1].prev, Link::Extra(0));
}

#[test]
#[should_panic]
fn test_remove_extra_value_panics_on_invalid_index() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 200, prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    let _ = remove_extra_value(raw_links, &mut extra_values, 3);
}

#[test]
fn test_remove_extra_value_updates_links_correctly() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 10])));
    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 200, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 300, prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    raw_links.0[0] = Some(Bucket { next: 1, tail: 2 });

    let removed = remove_extra_value(raw_links, &mut extra_values, 2);
    
    assert_eq!(removed.value, 300);
    assert_eq!(extra_values.len(), 2);
    assert_ne!(extra_values[0].next, Link::Entry(2));
}

