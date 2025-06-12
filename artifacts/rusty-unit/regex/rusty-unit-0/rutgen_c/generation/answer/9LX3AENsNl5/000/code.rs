// Answer 0

#[test]
fn test_usize_to_u32_within_limit() {
    let value: usize = 42;
    let result = usize_to_u32(value);
    assert_eq!(result, 42);
}

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_too_large() {
    let value: usize = 4294967296; // This is 2^32, which is out of u32 range
    let _result = usize_to_u32(value);
}

#[test]
fn test_usize_to_u32_boundary_value() {
    let value: usize = 4294967295; // This is 2^32 - 1, the maximum for u32
    let result = usize_to_u32(value);
    assert_eq!(result, 4294967295);
}

