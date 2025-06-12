// Answer 0

#[test]
fn test_u32_to_usize_within_bounds() {
    assert_eq!(u32_to_usize(0), 0);
    assert_eq!(u32_to_usize(1), 1);
    assert_eq!(u32_to_usize(usize::MAX as u32), usize::MAX);
}

#[test]
#[should_panic(expected = "BUG: 4294967295 is too big to be pointer sized")]
fn test_u32_to_usize_overflow() {
    // This tests the edge case where n is the maximum u32 value
    u32_to_usize(u32::MAX);
}

