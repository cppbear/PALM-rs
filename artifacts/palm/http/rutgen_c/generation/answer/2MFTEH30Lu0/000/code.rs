// Answer 0

#[test]
fn test_remove_extra_value_valid_index() {
    struct TestHeaderValue;
    let mut header_map = HeaderMap::<TestHeaderValue>::with_capacity(10);
    
    // Simulating the addition of extra values.
    header_map.extra_values.push(ExtraValue {
        value: TestHeaderValue,
        prev: Link::Empty,
        next: Link::Empty,
    });
    header_map.extra_values.push(ExtraValue {
        value: TestHeaderValue,
        prev: Link::Entry(0),
        next: Link::Empty,
    });
    
    // Remove the extra value at index 1
    let removed_extra = header_map.remove_extra_value(1);
    
    // Check if the removed value is the correct one.
    assert_eq!(header_map.extra_values.len(), 1);
    assert!(matches!(removed_extra.prev, Link::Entry(0)));
}

#[test]
#[should_panic]
fn test_remove_extra_value_out_of_bounds() {
    struct TestHeaderValue;
    let mut header_map = HeaderMap::<TestHeaderValue>::with_capacity(10);
    
    // Attempting to remove an extra value when none have been added yet.
    header_map.remove_extra_value(0);
}

#[test]
fn test_remove_extra_value_edge_case() {
    struct TestHeaderValue;
    let mut header_map = HeaderMap::<TestHeaderValue>::with_capacity(5);
    
    // Adding a single extra value
    header_map.extra_values.push(ExtraValue {
        value: TestHeaderValue,
        prev: Link::Empty,
        next: Link::Empty,
    });
    
    // Remove the only extra value
    let removed_extra = header_map.remove_extra_value(0);
    
    // Check if the extra_values are now empty
    assert!(header_map.extra_values.is_empty());
    assert!(matches!(removed_extra.prev, Link::Empty));
}

