// Answer 0

#[test]
#[should_panic]
fn test_read_u64_into_panics_when_src_is_too_small() {
    let src: &[u8] = &[1, 2, 3, 4, 5]; // Length is 5, not enough for even one u64
    let mut dst: [u64; 1] = [0]; // We need at least 8 bytes in src for one u64
    read_u64_into(src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_panics_when_src_is_exactly_8_bytes_for_two_u64s() {
    let src: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8]; // Length is 8
    let mut dst: [u64; 2] = [0; 2]; // Need 16 bytes in src
    read_u64_into(src, &mut dst);
}

#[test]
fn test_read_u64_into_succeeds_with_exact_length() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0, 
                       2, 0, 0, 0, 0, 0, 0, 0]; // 16 bytes for 2 u64s
    let mut dst: [u64; 2] = [0; 2]; // Prepare two u64s
    read_u64_into(src, &mut dst);
    assert_eq!(dst, [1, 2]);
}

#[test]
fn test_read_u64_into_succeeds_with_multiple_u64s() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0, 
                       2, 0, 0, 0, 0, 0, 0, 0,
                       3, 0, 0, 0, 0, 0, 0, 0,
                       4, 0, 0, 0, 0, 0, 0, 0]; // 32 bytes for 4 u64s
    let mut dst: [u64; 4] = [0; 4]; // Prepare four u64s
    read_u64_into(src, &mut dst);
    assert_eq!(dst, [1, 2, 3, 4]);
}

