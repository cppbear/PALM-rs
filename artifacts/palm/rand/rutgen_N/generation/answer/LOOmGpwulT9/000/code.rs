// Answer 0

#[test]
fn test_fill_via_u64_chunks_with_full_buffer() {
    let mut src = [0u64; 2]; // Enough u64 to fill dest completely
    let mut dest = [1u8; 16]; // 16 bytes to fill
    let (consumed, filled) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 2);
    assert_eq!(filled, 16);
}

#[test]
fn test_fill_via_u64_chunks_with_partial_buffer() {
    let mut src = [0u64; 1]; // Not enough u64 to fill dest completely
    let mut dest = [1u8; 12]; // 12 bytes to fill
    let (consumed, filled) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 2); // 12 bytes is 1.5 u64, round up to 2
    assert_eq!(filled, 12);
}

#[test]
fn test_fill_via_u64_chunks_with_empty_dest() {
    let mut src = [0u64; 1];
    let mut dest: [u8; 0] = [];
    let (consumed, filled) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 0);
    assert_eq!(filled, 0);
}

#[test]
fn test_fill_via_u64_chunks_with_no_src() {
    let mut src: [u64; 0] = [];
    let mut dest = [1u8; 8]; // 8 bytes to fill
    let (consumed, filled) = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 0);
    assert_eq!(filled, 0);
}

