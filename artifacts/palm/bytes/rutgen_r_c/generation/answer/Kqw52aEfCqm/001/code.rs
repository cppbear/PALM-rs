// Answer 0

#[test]
fn test_borrow_empty_bytes() {
    let bytes = Bytes::new();
    let result: &[u8] = bytes.borrow();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_borrow_non_empty_bytes() {
    let static_bytes: &'static [u8] = &[1, 2, 3, 4, 5];
    let bytes = Bytes::from_static(static_bytes);
    let result: &[u8] = bytes.borrow();
    assert_eq!(result, static_bytes);
}

#[test]
#[should_panic(expected = "slice out of bounds")]
fn test_borrow_after_slice_out_of_bounds() {
    let static_bytes: &'static [u8] = &[1, 2, 3, 4, 5];
    let mut bytes = Bytes::from_static(static_bytes);
    let result = bytes.slice(1..10); // Out of bounds slice
    result.borrow(); // Attempting to borrow should panic
}

