// Answer 0

#[test]
fn test_pos_invalid_index_too_large() {
    let locations = Locations(vec![Some(0), Some(1), Some(2)]);
    
    // Testing with an index that is out of bounds to trigger a None return.
    let result = locations.pos(3); // Index 3 would access out of bounds for the given vec.

    assert_eq!(result, None);
}

#[test]
fn test_pos_valid_index_with_no_match() {
    let locations = Locations(vec![Some(0), Some(1), None, Some(3), Some(4)]);
    
    // Testing with an index where one of the positions is None.
    let result = locations.pos(1); // This should result in None because the end position is None.

    assert_eq!(result, None);
}

#[test]
fn test_pos_invalid_index_empty() {
    let locations = Locations(vec![]);

    // Testing with an empty Locations to ensure it returns None.
    let result = locations.pos(0); // Index 0 is out of bounds for the empty vec.
    
    assert_eq!(result, None);
}

#[test]
fn test_pos_valid_index_but_end_none() {
    let locations = Locations(vec![Some(0), Some(1), Some(2), None]);

    // Testing with an index where the end position does not exist.
    let result = locations.pos(1); // Should return None since end position (2*1+1) is None.

    assert_eq!(result, None);
}

#[test]
fn test_pos_valid_index_matching() {
    let locations = Locations(vec![Some(0), Some(1), Some(2), Some(3)]);

    // Testing with a valid index that has both start and end positions filled.
    let result = locations.pos(1); // Both start (2*1=2) and end (2*1+1=3) should be Some.

    assert_eq!(result, Some((2, 3)));
}

