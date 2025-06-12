// Answer 0

#[test]
fn test_usize_to_u32_within_bounds() {
    let result = usize_to_u32(100);
    assert_eq!(result, 100);
}

#[test]
fn test_usize_to_u32_boundary_value() {
    let result = usize_to_u32(u32::MAX as usize);
    assert_eq!(result, u32::MAX);
}

#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
#[test]
fn test_usize_to_u32_overflow() {
    let _ = usize_to_u32(4294967296); // This is one more than u32::MAX
}

