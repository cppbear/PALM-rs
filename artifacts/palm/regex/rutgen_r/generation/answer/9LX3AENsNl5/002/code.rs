// Answer 0

#[test]
fn test_usize_to_u32_boundary() {
    let n: usize = std::u32::MAX as usize;
    let result = usize_to_u32(n);
    assert_eq!(result, std::u32::MAX);
}

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_too_large() {
    let n: usize = std::u32::MAX as usize + 1;
    usize_to_u32(n);
}

