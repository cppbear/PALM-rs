// Answer 0

#[test]
fn test_usize_to_u32_zero() {
    let n = 0;
    let result = usize_to_u32(n);
}

#[test]
fn test_usize_to_u32_max() {
    let n = 4294967295; // Maximum value that fits in u32
    let result = usize_to_u32(n);
}

#[test]
fn test_usize_to_u32_mid_range() {
    let n = 2147483648; // Mid-range value
    let result = usize_to_u32(n);
}

#[test]
fn test_usize_to_u32_boundary_minus_one() {
    let n = 4294967294; // Just below the maximum value
    let result = usize_to_u32(n);
}

#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
#[test]
fn test_usize_to_u32_too_large() {
    let n = 4294967296; // Just above the maximum value, should panic
    let result = usize_to_u32(n);
}

