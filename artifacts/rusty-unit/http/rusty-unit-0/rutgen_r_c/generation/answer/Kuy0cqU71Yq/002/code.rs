// Answer 0

#[test]
fn test_reinsert_entry_in_order_empty_bucket() {
    // Given
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(512);
    let pos = Pos::new(1, HashValue(0)); // Valid position

    // Ensure that header_map.indices has at least one element
    header_map.indices = vec![Pos::none().into(); 512].into_boxed_slice();

    // When
    header_map.reinsert_entry_in_order(pos);

    // Then
    assert_eq!(header_map.indices[desired_pos(header_map.mask, pos.hash)].resolve(), Some((1, HashValue(0))));
}

#[test]
fn test_reinsert_entry_in_order_non_empty_bucket() {
    // Given
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(512);
    let pos1 = Pos::new(1, HashValue(0)); // Valid position
    let pos2 = Pos::new(2, HashValue(1)); // Another valid position
    
    // Ensure that header_map.indices has at least two elements
    header_map.indices = vec![Pos::none().into(); 512].into_boxed_slice();
    
    // Insert the first position
    header_map.indices[desired_pos(header_map.mask, pos1.hash)] = pos1;

    // When
    header_map.reinsert_entry_in_order(pos2);

    // Then
    assert_eq!(header_map.indices[desired_pos(header_map.mask, pos2.hash)].resolve(), Some((2, HashValue(1))));
}

