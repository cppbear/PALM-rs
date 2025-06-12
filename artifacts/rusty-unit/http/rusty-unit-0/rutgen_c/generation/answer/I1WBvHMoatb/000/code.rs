// Answer 0

#[test]
fn test_drain_all_extra_values_single_entry() {
    struct TestBucket;
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Extra(3) },
        ExtraValue { value: 4, prev: Link::Extra(2), next: Link::Extra(3) },
        ExtraValue { value: 5, prev: Link::Extra(3), next: Link::Entry(1) },
        ExtraValue { value: 6, prev: Link::Entry(1), next: Link::Extra(0) }
    ];
    
    let raw_links = RawLinks(ptr::null_mut());

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
    
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_drain_all_extra_values_empty() {
    let extra_values: &mut Vec<ExtraValue<i32>> = &mut Vec::new();
    let raw_links = RawLinks(ptr::null_mut());

    let result = drain_all_extra_values(raw_links, extra_values, 0);

    assert!(result.is_empty());
}

#[test]
fn test_drain_all_extra_values_multiple_linked() {
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 30, prev: Link::Extra(1), next: Link::Entry(1) }
    ];

    let raw_links = RawLinks(ptr::null_mut());

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);

    assert_eq!(result, vec![10, 20, 30]);
}

#[test]
#[should_panic]
fn test_drain_all_extra_values_invalid_index() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(2) },
    ];
    
    let raw_links = RawLinks(ptr::null_mut());

    // This is expected to panic due to invalid index access
    drain_all_extra_values(raw_links, &mut extra_values, 10);
}

