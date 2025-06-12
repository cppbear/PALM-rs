// Answer 0

#[test]
fn test_read_u64_into_exact_fit() {
    let src: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 1] = [0];
    read_u64_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_insufficient_src_length() {
    let src: [u8; 7] = [1, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 1] = [0];
    read_u64_into(&src, &mut dst);
}

#[test]
fn test_read_u64_into_multiple_chunks() {
    let src: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 2] = [0, 0];
    read_u64_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_zero_length_dst() {
    let src: [u8; 0] = [];
    let mut dst: [u64; 0] = [];
    read_u64_into(&src, &mut dst);
}

#[test]
fn test_read_u64_into_exact_fit_with_zeroes() {
    let src: [u8; 8] = [0; 8];
    let mut dst: [u64; 1] = [0];
    read_u64_into(&src, &mut dst);
}

