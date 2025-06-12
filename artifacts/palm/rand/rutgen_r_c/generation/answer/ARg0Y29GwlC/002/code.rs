// Answer 0

#[test]
fn test_read_u64_into_valid_input() {
    let src: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 2] = [0; 2];
    read_u64_into(&src, &mut dst);
    assert_eq!(dst, [1, 2]);
}

#[test]
#[should_panic]
fn test_read_u64_into_src_too_short() {
    let src: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 2] = [0; 2];
    read_u64_into(&src, &mut dst); // Should panic because src.len() < 8 * dst.len()
}

#[test]
fn test_read_u64_into_exact_boundary() {
    let src: [u8; 24] = [
        1, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut dst: [u64; 3] = [0; 3];
    read_u64_into(&src, &mut dst);
    assert_eq!(dst, [1, 2, 3]);
}

#[test]
fn test_read_u64_into_empty_dst() {
    let src: [u8; 0] = [];
    let mut dst: [u64; 0] = [];
    read_u64_into(&src, &mut dst); // Should not panic, as dst is empty
}

#[test]
#[should_panic]
fn test_read_u64_into_src_empty_dst_nonempty() {
    let src: [u8; 0] = [];
    let mut dst: [u64; 1] = [0];
    read_u64_into(&src, &mut dst); // Should panic because src.len() < 8 * dst.len()
}

