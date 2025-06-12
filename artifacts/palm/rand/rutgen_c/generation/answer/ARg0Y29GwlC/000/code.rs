// Answer 0

#[test]
fn test_read_u64_into_basic() {
    let src: &[u8] = &[
        1, 0, 0, 0, 0, 0, 0, 0, 
        2, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut dst: [u64; 3] = [0; 3];
    
    read_u64_into(src, &mut dst);
    
    assert_eq!(dst, [1, 2, 3]);
}

#[test]
#[should_panic]
fn test_read_u64_into_insufficient_length() {
    let src: &[u8] = &[
        1, 0, 0, 0, 0, 0, 0, 0, 
        2, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut dst: [u64; 3] = [0; 3];
    
    read_u64_into(src, &mut dst);
}

#[test]
fn test_read_u64_into_exact_boundary() {
    let src: &[u8] = &[
        1, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0,
        4, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut dst: [u64; 4] = [0; 4];
    
    read_u64_into(src, &mut dst);
    
    assert_eq!(dst, [1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_read_u64_into_too_short_src() {
    let src: &[u8] = &[1, 0, 0, 0, 0, 0, 0, 0]; // Only enough for 1 u64
    let mut dst: [u64; 2] = [0; 2];
    
    read_u64_into(src, &mut dst);
}

