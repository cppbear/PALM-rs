// Answer 0

#[test]
fn test_read_u32_into_exact_length() {
    let src: Vec<u8> = vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]; // length 16 bytes
    let mut dst: [u32; 4] = [0; 4]; // length 4

    read_u32_into(&src, &mut dst);

    assert_eq!(dst, [1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_read_u32_into_insufficient_length() {
    let src: Vec<u8> = vec![1, 0, 0, 0, 2, 0, 0, 0]; // length 8 bytes
    let mut dst: [u32; 3] = [0; 3]; // length 3

    // This should panic because 8 < 4 * 3
    read_u32_into(&src, &mut dst);
}

#[test]
fn test_read_u32_into_empty_dst() {
    let src: Vec<u8> = vec![1, 0, 0, 0]; // length 4 bytes
    let mut dst: [u32; 0] = []; // length 0

    read_u32_into(&src, &mut dst);

    assert_eq!(dst.len(), 0);
}

#[test]
fn test_read_u32_into_zero_length_src() {
    let src: Vec<u8> = vec![]; // length 0 bytes
    let mut dst: [u32; 0] = []; // length 0

    read_u32_into(&src, &mut dst);

    assert_eq!(dst.len(), 0);
}

#[test]
fn test_read_u32_into_maximum_size() {
    let src: Vec<u8> = (0..u32::MAX).flat_map(|i| i.to_le_bytes()).collect(); // large src
    let mut dst: [u32; 1] = [0]; // length 1

    read_u32_into(&src[..4], &mut dst); // Use first 4 bytes for a single u32

    assert_eq!(dst[0], 0);
}

