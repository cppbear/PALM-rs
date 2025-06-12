// Answer 0

#[test]
fn test_usize_to_u32_max_bound() {
    let input: usize = std::u32::MAX as usize; // Max value fitting into u32
    let result = usize_to_u32(input);
    assert_eq!(result, std::u32::MAX);
}

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_exceeding_max() {
    let input: usize = std::u32::MAX as usize + 1; // Just above the maximum u32 value
    let _ = usize_to_u32(input);
}

