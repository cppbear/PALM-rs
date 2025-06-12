// Answer 0

#[test]
fn test_usize_to_u32_panics_on_large_input_1() {
    let _ = usize_to_u32(4294967296);
}

#[test]
fn test_usize_to_u32_panics_on_large_input_2() {
    let _ = usize_to_u32(4294967297);
}

#[test]
fn test_usize_to_u32_panics_on_large_input_3() {
    let _ = usize_to_u32(usize::MAX);
}

