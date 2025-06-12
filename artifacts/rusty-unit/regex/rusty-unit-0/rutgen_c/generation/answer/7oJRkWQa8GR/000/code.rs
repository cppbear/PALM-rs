// Answer 0

#[test]
fn test_u32_to_usize_within_range() {
    let result = u32_to_usize(10);
    assert_eq!(result, 10);
}

#[test]
fn test_u32_to_usize_boundary_value() {
    let result = u32_to_usize(std::usize::MAX as u32);
    assert_eq!(result, std::usize::MAX as u32);
}

#[should_panic(expected = "BUG: 4294967296 is too big to be pointer sized")]
#[test]
fn test_u32_to_usize_overflow() {
    let _ = u32_to_usize(4294967296);
}

