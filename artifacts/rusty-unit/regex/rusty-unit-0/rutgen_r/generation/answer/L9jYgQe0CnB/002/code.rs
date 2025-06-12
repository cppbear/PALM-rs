// Answer 0

#[test]
fn test_usize_to_u32_max_bound() {
    let n: usize = std::u32::MAX as usize; // This should not panic and should return u32::MAX
    assert_eq!(usize_to_u32(n), std::u32::MAX);
}

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_exceed_bound() {
    let n: usize = std::u32::MAX as usize + 1; // This should panic
    usize_to_u32(n);
}

#[test]
fn test_usize_to_u32_zero() {
    let n: usize = 0; // This should return 0
    assert_eq!(usize_to_u32(n), 0);
}

#[test]
fn test_usize_to_u32_small_value() {
    let n: usize = 1; // This should return 1
    assert_eq!(usize_to_u32(n), 1);
}

#[test]
fn test_usize_to_u32_large_value() {
    let n: usize = 2; // This should return 2
    assert_eq!(usize_to_u32(n), 2);
}

