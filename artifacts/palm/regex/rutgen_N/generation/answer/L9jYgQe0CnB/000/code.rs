// Answer 0

#[test]
fn test_usize_to_u32_within_bounds() {
    let result = usize_to_u32(42);
    assert_eq!(result, 42);
}

#[test]
fn test_usize_to_u32_upper_bound() {
    let result = usize_to_u32(::std::u32::MAX as usize);
    assert_eq!(result, ::std::u32::MAX);
}

#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
#[test]
fn test_usize_to_u32_over_upper_bound() {
    let _ = usize_to_u32(::std::u32::MAX as usize + 1);
}

