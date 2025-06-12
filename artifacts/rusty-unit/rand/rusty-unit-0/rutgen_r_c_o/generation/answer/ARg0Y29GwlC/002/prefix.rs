// Answer 0

#[test]
fn test_read_u64_into_basic() {
    let src: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 2] = [0; 2];
    read_u64_into(&src, &mut dst);
}

#[test]
fn test_read_u64_into_edge_case_minimum() {
    let src: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 1] = [0; 1];
    read_u64_into(&src, &mut dst);
}

#[test]
fn test_read_u64_into_edge_case_maximum() {
    let src: [u8; 24] = [
        1, 0, 0, 0, 0, 0, 0, 0, 
        2, 0, 0, 0, 0, 0, 0, 0, 
        3, 0, 0, 0, 0, 0, 0, 0
    ];
    let mut dst: [u64; 3] = [0; 3];
    read_u64_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_panic_src_too_short() {
    let src: [u8; 7] = [1, 0, 0, 0, 0, 0, 0];
    let mut dst: [u64; 1] = [0; 1];
    read_u64_into(&src, &mut dst);
}

#[test]
fn test_read_u64_into_large_data() {
    let src: [u8; 64] = [
        1, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0,
        4, 0, 0, 0, 0, 0, 0, 0,
        5, 0, 0, 0, 0, 0, 0, 0,
        6, 0, 0, 0, 0, 0, 0, 0,
        7, 0, 0, 0, 0, 0, 0, 0,
        8, 0, 0, 0, 0, 0, 0, 0
    ];
    let mut dst: [u64; 8] = [0; 8];
    read_u64_into(&src, &mut dst);
}

#[test]
fn test_read_u64_into_empty() {
    let src: [u8; 0] = [];
    let mut dst: [u64; 0] = [];
    read_u64_into(&src, &mut dst);
}

