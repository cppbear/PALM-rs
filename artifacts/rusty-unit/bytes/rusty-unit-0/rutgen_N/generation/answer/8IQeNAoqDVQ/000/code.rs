// Answer 0

#[test]
fn test_offset_from_equal_addresses() {
    let ptr: *const u8 = &0u8; // Pointer to a single byte
    let dst: *const u8 = ptr; // Same address as original
    assert_eq!(offset_from(dst, ptr), 0);
}

#[test]
fn test_offset_from_different_addresses() {
    let original: [u8; 2] = [0, 1]; // An array to create distinct addresses
    let ptr: *const u8 = original.as_ptr(); // Pointer to the original array
    let dst: *const u8 = unsafe { ptr.add(1) }; // Pointer to the second element
    assert_eq!(offset_from(dst, ptr), 1);
}

#[test]
fn test_offset_from_non_zero_addresses() {
    let original: [u8; 3] = [0, 1, 2]; // An array to create distinct addresses
    let ptr: *const u8 = original.as_ptr(); // Pointer to the original array
    let dst: *const u8 = unsafe { ptr.add(2) }; // Pointer to the third element
    assert_eq!(offset_from(dst, ptr), 2);
}

#[should_panic]
fn test_offset_from_invalid_pointer() {
    let ptr: *const u8 = 0 as *const u8; // Null pointer
    let dst: *const u8 = 1 as *const u8; // Another arbitrary pointer
    offset_from(dst, ptr);
}

