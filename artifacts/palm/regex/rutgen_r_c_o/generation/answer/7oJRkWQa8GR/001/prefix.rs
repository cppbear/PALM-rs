// Answer 0

#[test]
fn test_u32_to_usize_below_overflow() {
    let _ = u32_to_usize(4294967295);
}

#[should_panic]
fn test_u32_to_usize_overflow() {
    let _ = u32_to_usize(4294967296);
}

