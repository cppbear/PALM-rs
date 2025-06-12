// Answer 0

#[test]
#[should_panic]
fn test_remove_extra_value_index_out_of_bounds() {
    struct DummyBucket;
    
    let raw_links = RawLinks(Box::into_raw(Box::new([None; 1]))); // Simulating a raw link with one bucket
    let mut extra_values = vec![
        ExtraValue { value: DummyBucket, prev: Link::Extra(0), next: Link::Extra(0) } // One entry to exceed index
    ];
    
    // Attempting to remove the extra value at an index equal to the length of extra_values should panic
    let _ = remove_extra_value(raw_links, &mut extra_values, extra_values.len());
}

#[test]
fn test_remove_extra_value_correct_removal() {
    struct DummyBucket;
    
    let mut extra_values = vec![
        ExtraValue { value: DummyBucket, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: DummyBucket, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: DummyBucket, prev: Link::Entry(1), next: Link::Extra(0) },
    ];

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Links { next: 1, tail: 1 }), None, Some(Links { next: 0, tail: 0 })])));
    
    // Remove ExtraValue at index 1
    let removed = remove_extra_value(raw_links, &mut extra_values, 1);
    
    // Verifying the returned extra value
    assert_eq!(removed.value, DummyBucket);
    assert_eq!(extra_values.len(), 2);
    
    // Verifying the links for correct adjustments
    assert_eq!(extra_values[0].next, Link::Entry(1));
    assert_eq!(extra_values[2].prev, Link::Extra(0));
}

#[test]
fn test_remove_extra_value_last_element() {
    struct DummyBucket;

    let mut extra_values = vec![
        ExtraValue { value: DummyBucket, prev: Link::Extra(0), next: Link::Entry(0) },
    ];

    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 1])));
    
    // Remove ExtraValue at index 0
    let removed = remove_extra_value(raw_links, &mut extra_values, 0);
    
    // Verifying the returned extra value
    assert_eq!(removed.value, DummyBucket);
    assert_eq!(extra_values.len(), 0);
}

