// Answer 0

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_too_big() {
    let input: usize = 4294967296; // This is 2^32, which is just above u32::MAX
    usize_to_u32(input);
}

#[test]
fn test_usize_to_u32_max() {
    let input: usize = 4294967295; // This is u32::MAX
    let result = usize_to_u32(input);
    assert_eq!(result, 4294967295);
}

#[test]
fn test_usize_to_u32_zero() {
    let input: usize = 0; // Testing the lower boundary
    let result = usize_to_u32(input);
    assert_eq!(result, 0);
}

#[test]
fn test_usize_to_u32_small_value() {
    let input: usize = 1; // Testing a small valid input
    let result = usize_to_u32(input);
    assert_eq!(result, 1);
}

#[test]
fn test_usize_to_u32_large_value() {
    let input: usize = 123456789; // Testing a random large valid input
    let result = usize_to_u32(input);
    assert_eq!(result, 123456789);
}

