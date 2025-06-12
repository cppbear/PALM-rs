// Answer 0

#[test]
#[should_panic]
fn test_read_u32_into_insufficient_src_length() {
    let src: &[u8] = &[1, 2, 3]; // Length is 3, which is less than 4 * dst.len() where dst.len() = 1.
    let mut dst: [u32; 1] = [0];
    read_u32_into(src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_empty_dst() {
    let src: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8]; // Sufficient length for dst.len() = 0.
    let mut dst: [u32; 0] = [];
    read_u32_into(src, &mut dst);
}

