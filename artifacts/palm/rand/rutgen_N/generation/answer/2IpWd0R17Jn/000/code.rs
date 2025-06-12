// Answer 0

#[test]
fn test_read_u32_into_exact_length() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0];
    let mut dst: [u32; 2] = [0; 2];
    read_u32_into(src, &mut dst);
    assert_eq!(dst, [1, 2]);
}

#[test]
fn test_read_u32_into_boundary_length() {
    let src: &[u8] = &[1, 0, 0, 0];
    let mut dst: [u32; 1] = [0; 1];
    read_u32_into(src, &mut dst);
    assert_eq!(dst, [1]);
}

#[should_panic]
fn test_read_u32_into_insufficient_length() {
    let src: &[u8] = &[1, 0, 0]; // insufficient data for any u32
    let mut dst: [u32; 1] = [0; 1];
    read_u32_into(src, &mut dst);
}

#[test]
fn test_read_u32_into_multiple_chunks() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0];
    let mut dst: [u32; 4] = [0; 4];
    read_u32_into(src, &mut dst);
    assert_eq!(dst, [1, 2, 3, 4]);
}

