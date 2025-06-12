// Answer 0

#[test]
fn test_read_u32_into_normal_case() {
    let src: Vec<u8> = vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0];
    let mut dst: [u32; 3] = [0; 3];
    read_u32_into(&src, &mut dst);
    assert_eq!(dst, [1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32_into_insufficient_source_length() {
    let src: Vec<u8> = vec![1, 0, 0, 0];
    let mut dst: [u32; 2] = [0; 2];
    read_u32_into(&src, &mut dst);
}

#[test]
fn test_read_u32_into_boundary_source_length() {
    let src: Vec<u8> = vec![4, 0, 0, 0, 5, 0, 0, 0];
    let mut dst: [u32; 2] = [0; 2];
    read_u32_into(&src, &mut dst);
    assert_eq!(dst, [4, 5]);
}

#[test]
fn test_read_u32_into_empty_dst() {
    let src: Vec<u8> = vec![];
    let mut dst: [u32; 0] = [];
    read_u32_into(&src, &mut dst);
}


