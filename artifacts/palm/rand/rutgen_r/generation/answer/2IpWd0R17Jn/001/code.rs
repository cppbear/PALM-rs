// Answer 0

#[test]
fn test_read_u32_into_valid_input() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0];
    let mut dst = [0u32; 4];
    
    read_u32_into(src, &mut dst);
    
    assert_eq!(dst, [1, 2, 3, 4]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32_into_insufficient_src_length() {
    let src: &[u8] = &[1, 0, 0, 0, 2, 0, 0, 0];
    let mut dst = [0u32; 3];
    
    read_u32_into(src, &mut dst);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_read_u32_into_invalid_chunk_size() {
    let src: &[u8] = &[1, 0, 0];  // Not enough bytes for one u32
    let mut dst = [0u32; 1];

    read_u32_into(src, &mut dst);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_read_u32_into_empty_src() {
    let src: &[u8] = &[];
    let mut dst = [0u32; 1]; // One u32 would require 4 bytes

    read_u32_into(src, &mut dst);
}

