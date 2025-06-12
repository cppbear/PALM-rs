// Answer 0

#[test]
fn test_read_u32_into_basic() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0];
    let mut dst: [u32; 4] = [0; 4];
    read_u32_into(src, &mut dst);
    assert_eq!(dst, [1, 2, 3, 4]);
}

#[test]
fn test_read_u32_into_edge_case() {
    let src: &[u8] = &[0, 0, 0, 0]; // Edge case with exactly enough bytes for one u32
    let mut dst: [u32; 1] = [0; 1];
    read_u32_into(src, &mut dst);
    assert_eq!(dst, [0]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32_into_panic_too_short() {
    let src: &[u8] = &[1, 0, 0]; // Too short for any u32
    let mut dst: [u32; 1] = [0; 1];
    read_u32_into(src, &mut dst);
} 

#[test]
fn test_read_u32_into_multiple() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0];
    let mut dst: [u32; 5] = [0; 5];
    read_u32_into(src, &mut dst);
    assert_eq!(dst, [1, 2, 3, 4, 5]);
}

