// Answer 0

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_panic_over_max() {
    let n: usize = 4294967296; // This value is just over the u32::MAX
    usize_to_u32(n);
}

#[test]
fn test_usize_to_u32_max() {
    let n: usize = 4294967295; // This value is equal to u32::MAX
    let result = usize_to_u32(n);
    assert_eq!(result, 4294967295);
}

#[test]
fn test_usize_to_u32_zero() {
    let n: usize = 0; // Lower boundary value
    let result = usize_to_u32(n);
    assert_eq!(result, 0);
}

#[test]
fn test_usize_to_u32_small_value() {
    let n: usize = 123; // A small value within u32 range
    let result = usize_to_u32(n);
    assert_eq!(result, 123);
}

