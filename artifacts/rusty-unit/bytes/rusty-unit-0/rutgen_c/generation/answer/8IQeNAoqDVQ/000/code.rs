// Answer 0

#[test]
fn test_offset_from_equal_pointers() {
    let ptr: *const u8 = &0u8;
    let expected = 0usize;
    let result = offset_from(ptr, ptr);
    assert_eq!(result, expected);
}

#[test]
fn test_offset_from_different_pointers() {
    let original: u8 = 0;
    let dst: u8 = 1;

    let original_ptr: *const u8 = &original;
    let dst_ptr: *const u8 = &dst;

    let expected = 1usize;
    let result = offset_from(dst_ptr, original_ptr);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_offset_from_dst_less_than_original() {
    let original: u8 = 2;
    let dst: u8 = 1;

    let original_ptr: *const u8 = &original;
    let dst_ptr: *const u8 = &dst;

    // This case should panic because the precondition is violated (dst < original)
    let _ = offset_from(dst_ptr, original_ptr);
}

