// Answer 0

#[test]
fn test_usize_to_u32_with_max_value() {
    let n: usize = std::u32::MAX as usize; // Maximum value that fits in u32
    let result = usize_to_u32(n);
    assert_eq!(result, std::u32::MAX);
}

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_exceeding_max_value() {
    let n: usize = std::u32::MAX as usize + 1; // Exceeding the max value for u32
    usize_to_u32(n);
}

#[test]
fn test_usize_to_u32_with_zero() {
    let n: usize = 0; // Testing with boundary value zero
    let result = usize_to_u32(n);
    assert_eq!(result, 0);
}

#[test]
fn test_usize_to_u32_with_small_value() {
    let n: usize = 123; // Testing with a small valid value
    let result = usize_to_u32(n);
    assert_eq!(result, 123);
}

