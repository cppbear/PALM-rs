// Answer 0

#[test]
fn test_usize_to_u32_within_bounds() {
    let result = usize_to_u32(42);
    assert_eq!(result, 42);
}

#[test]
fn test_usize_to_u32_lower_bound() {
    let result = usize_to_u32(0);
    assert_eq!(result, 0);
}

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_upper_bound() {
    usize_to_u32(4294967296);
}

#[test]
fn test_usize_to_u32_edge_case() {
    let result = usize_to_u32(4294967295);
    assert_eq!(result, 4294967295);
}

