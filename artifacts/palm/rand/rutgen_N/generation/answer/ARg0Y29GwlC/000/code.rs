// Answer 0

#[test]
fn test_read_u64_into_success() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 2] = [0; 2];
    read_u64_into(src, &mut dst);
    assert_eq!(dst, [1, 2]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u64_into_insufficient_length() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0]; // Only enough for 1 u64
    let mut dst: [u64; 2] = [0; 2]; // Need space for 2 u64s
    read_u64_into(src, &mut dst); // This should panic
}

#[test]
fn test_read_u64_into_boundary_case() {
    let src: &[u8] = &[
        0, 0, 0, 0, 0, 0, 0, 1, 
        0, 0, 0, 0, 0, 0, 0, 2,
        0, 0, 0, 0, 0, 0, 0, 3,
    ];
    let mut dst: [u64; 3] = [0; 3];
    read_u64_into(src, &mut dst);
    assert_eq!(dst, [1, 2, 3]);
}

