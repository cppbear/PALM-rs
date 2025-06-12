// Answer 0

#[test]
fn test_fill_via_u32_chunks_empty_dest() {
    let mut src: &mut [u32] = &mut [];
    let mut dest: &mut [u8] = &mut [];
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(src, dest);
    assert_eq!(consumed_u32, 0);
    assert_eq!(filled_u8, 0);
}

#[test]
fn test_fill_via_u32_chunks_small_dest() {
    let mut src: &mut [u32] = &mut [1, 2, 3, 4];
    let mut dest: &mut [u8] = &mut [0; 3];
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(src, dest);
    assert_eq!(consumed_u32, 1);
    assert_eq!(filled_u8, 3);
    assert_eq!(dest, &[0, 0, 0]);
}

#[test]
fn test_fill_via_u32_chunks_exact_fill() {
    let mut src: &mut [u32] = &mut [0x01020304, 0x05060708];
    let mut dest: &mut [u8] = &mut [0; 8];
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(src, dest);
    assert_eq!(consumed_u32, 2);
    assert_eq!(filled_u8, 8);
    assert_eq!(dest, &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_fill_via_u32_chunks_partial_fill() {
    let mut src: &mut [u32] = &mut [0x01020304, 0x05060708];
    let mut dest: &mut [u8] = &mut [0; 5];
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(src, dest);
    assert_eq!(consumed_u32, 1);
    assert_eq!(filled_u8, 4); // Filling will stop after 4 bytes, leaving the array unchanged
    assert_eq!(dest, &[1, 2, 3, 4, 0]);
}

#[test]
fn test_fill_via_u32_chunks_large_dest() {
    let mut src: &mut [u32] = &mut [0x01020304; 100];
    let mut dest: &mut [u8] = &mut [0; 404]; // 100 * 4 = 400 bytes
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(src, dest);
    assert_eq!(consumed_u32, 100);
    assert_eq!(filled_u8, 400);
    assert_eq!(dest[..4], &[1, 2, 3, 4]); // Check first bytes
    assert_eq!(dest[396..], &[0, 0, 0, 0]); // Check the tail
}

