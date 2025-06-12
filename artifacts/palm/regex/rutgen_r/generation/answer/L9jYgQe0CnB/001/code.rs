// Answer 0

#[test]
#[should_panic(expected = "BUG: 4294967296 is too big to fit into u32")]
fn test_usize_to_u32_too_big() {
    let n: usize = 4294967296; // u32::MAX + 1
    usize_to_u32(n);
}

#[test]
#[should_panic(expected = "BUG: 18446744073709551616 is too big to fit into u32")]
fn test_usize_to_u32_too_big_large_value() {
    let n: usize = 18446744073709551616; // 2^64
    usize_to_u32(n);
}

#[test]
#[should_panic(expected = "BUG: 4294967295 is too big to fit into u32")]
fn test_usize_to_u32_max_u32() {
    let n: usize = 4294967295; // u32::MAX
    usize_to_u32(n);
}

