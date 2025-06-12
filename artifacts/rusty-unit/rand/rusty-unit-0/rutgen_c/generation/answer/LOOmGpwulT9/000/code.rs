// Answer 0

#[test]
fn test_fill_via_u64_chunks_full_fill() {
    let mut src: [u64; 2] = [1_u64, 2_u64]; // 16 bytes total when converted to bytes
    let mut dest: [u8; 16] = [0; 16];
    let (consumed_u64, filled_u8) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed_u64, 2);
    assert_eq!(filled_u8, 16);
    assert_eq!(dest, [1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_via_u64_chunks_partial_fill() {
    let mut src: [u64; 1] = [1_u64]; // 8 bytes total when converted to bytes
    let mut dest: [u8; 10] = [0; 10];
    let (consumed_u64, filled_u8) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed_u64, 2); // 10 bytes filled means 2 u64 are consumed
    assert_eq!(filled_u8, 10);
    assert_eq!(dest, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_via_u64_chunks_empty_dest() {
    let mut src: [u64; 1] = [1_u64];
    let mut dest: [u8; 0] = [];
    let (consumed_u64, filled_u8) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed_u64, 0);
    assert_eq!(filled_u8, 0);
}

#[test]
fn test_fill_via_u64_chunks_small_dest() {
    let mut src: [u64; 1] = [1_u64];
    let mut dest: [u8; 5] = [0; 5];
    let (consumed_u64, filled_u8) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed_u64, 1); // 5 bytes filled means 1 u64 is consumed
    assert_eq!(filled_u8, 5);
    assert_eq!(dest, [1, 0, 0, 0, 0]);
}

