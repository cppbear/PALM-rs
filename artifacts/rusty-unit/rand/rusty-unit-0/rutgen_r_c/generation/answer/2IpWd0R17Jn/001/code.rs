// Answer 0

#[test]
fn test_read_u32_into_valid_case() {
    let src: Vec<u8> = vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0];
    let mut dst: Vec<u32> = vec![0; 4];
    read_u32_into(&src, &mut dst);
    assert_eq!(dst, vec![1, 2, 3, 4]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32_into_src_too_short() {
    let src: Vec<u8> = vec![1, 0, 0, 0, 2, 0, 0, 0]; // 8 bytes
    let mut dst: Vec<u32> = vec![0; 3]; // needs 12 bytes in src
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32_into_zero_length_dst() {
    let src: Vec<u8> = vec![1, 0, 0, 0, 2, 0, 0, 0];
    let mut dst: Vec<u32> = vec![]; // 0 length dst
    read_u32_into(&src, &mut dst); // should panic since src.len() < 4 * dst.len()
}

#[test]
fn test_read_u32_into_dst_length_one() {
    let src: Vec<u8> = vec![255, 255, 255, 255]; // 1 u32
    let mut dst: Vec<u32> = vec![0; 1];
    read_u32_into(&src, &mut dst);
    assert_eq!(dst, vec![4294967295]); // 255 in LE converts to 0xFFFFFFFF
}

#[test]
fn test_read_u32_into_dst_length_two() {
    let src: Vec<u8> = vec![1, 0, 0, 0, 2, 0, 0, 0]; // 2 u32s
    let mut dst: Vec<u32> = vec![0; 2];
    read_u32_into(&src, &mut dst);
    assert_eq!(dst, vec![1, 2]);
}

