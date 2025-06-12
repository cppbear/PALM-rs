// Answer 0

#[test]
fn test_remove_extra_value_simple() {
    struct TestBucket;
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket), None, Some(TestBucket)])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Entry(2) },
        ExtraValue { value: 3, prev: Link::Entry(1), next: Link::Extra(3) },
    ];
    
    let idx = 1;
    let extra_value = remove_extra_value(raw_links, &mut extra_values, idx);
    assert_eq!(extra_value.value, 2);
    assert_eq!(extra_values.len(), 2);
}

#[test]
fn test_remove_extra_value_with_displacement() {
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None, Some(TestBucket)])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Entry(2) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(2) },
        ExtraValue { value: 3, prev: Link::Extra(0), next: Link::Extra(1) },
    ];

    let idx = 0;
    let extra_value = remove_extra_value(raw_links, &mut extra_values, idx);
    assert_eq!(extra_value.value, 1);
    assert_eq!(extra_values.len(), 2);
}

#[test]
#[should_panic]
fn test_remove_extra_value_out_of_bounds() {
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket)])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(0) },
    ];

    let idx = 2; // Out of bounds
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_unlinked() {
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket), Some(TestBucket)])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Entry(0) },
    ];

    let idx = 1;
    let extra_value = remove_extra_value(raw_links, &mut extra_values, idx);
    assert_eq!(extra_value.value, 2);
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_prev_entry() {
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket)])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(1), next: Link::Entry(0) }, // Invalid prev
    ];

    let idx = 0;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

