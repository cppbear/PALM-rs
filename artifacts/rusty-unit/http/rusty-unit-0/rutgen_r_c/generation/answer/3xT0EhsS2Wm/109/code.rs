// Answer 0

#[test]
fn test_remove_extra_value_with_valid_index() {
    // Prepare the necessary data structures for the test
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None; 5])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) }, // idx 0
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Entry(1) }, // idx 1
        ExtraValue { value: 3, prev: Link::Entry(1), next: Link::Extra(3) }, // idx 2
        ExtraValue { value: 4, prev: Link::Extra(2), next: Link::Entry(3) }, // idx 3
        ExtraValue { value: 5, prev: Link::Entry(3), next: Link::Extra(4) }, // idx 4
    ];

    // Test removing the value at index 1
    let removed = remove_extra_value(raw_links, &mut extra_values, 1);

    // Check that the removed value is as expected
    assert_eq!(removed.value, 2);
    assert!(extra_values.len() == 4);
    assert_eq!(extra_values[0].next, Link::Entry(1)); // The link remains correct
}

#[test]
fn test_remove_extra_value_with_boundary_index() {
    // Prepare the necessary data structures for the test
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([Some(Links { next: 1, tail: 2 }); 5])));
    let mut extra_values = vec![
        ExtraValue { value: 6, prev: Link::Extra(1), next: Link::Entry(0) }, // idx 0
        ExtraValue { value: 7, prev: Link::Entry(0), next: Link::Entry(1) }, // idx 1
        ExtraValue { value: 8, prev: Link::Entry(1), next: Link::Extra(3) }, // idx 2
        ExtraValue { value: 9, prev: Link::Extra(2), next: Link::Entry(3) }, // idx 3
    ];

    // Test removing the value at the last index (boundary condition)
    let removed = remove_extra_value(raw_links, &mut extra_values, 3);
    
    // Check that the removed value is as expected
    assert_eq!(removed.value, 9);
    assert!(extra_values.len() == 3);
}

#[should_panic]
#[test]
fn test_remove_extra_value_index_out_of_bounds() {
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None; 5])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) }, // idx 0
    ];

    // Attempt to remove an extra value with an invalid index, should panic
    let _ = remove_extra_value(raw_links, &mut extra_values, 2);
}

#[test]
fn test_remove_extra_value_with_prev_as_length() {
    // Prepare the necessary data structures for the test
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None; 5])));
    let mut extra_values = vec![
        ExtraValue { value: 11, prev: Link::Extra(1), next: Link::Entry(0) }, // idx 0
        ExtraValue { value: 12, prev: Link::Entry(0), next: Link::Entry(1) }, // idx 1
    ];

    // Test removing the last value (prev equals length)
    let removed = remove_extra_value(raw_links, &mut extra_values, 1);

    // Check that the removed value is as expected
    assert_eq!(removed.value, 12);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].next, Link::Entry(0)); // The link remains correct
}

