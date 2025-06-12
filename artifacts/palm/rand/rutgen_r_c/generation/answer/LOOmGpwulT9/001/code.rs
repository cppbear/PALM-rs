// Answer 0

#[test]
fn test_fill_via_u64_chunks_empty_src() {
    let mut src: [u64; 0] = [];
    let mut dest: [u8; 8] = [0; 8];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_fill_via_u64_chunks_partial_fill() {
    let mut src: [u64; 1] = [1];
    let mut dest: [u8; 8] = [0; 5]; // Only 5 bytes available to fill
    let result = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(result, (1, 5));
    assert_eq!(dest, [1, 0, 0, 0, 0]);
}

#[test]
fn test_fill_via_u64_chunks_exact_fill() {
    let mut src: [u64; 1] = [0x0102030405060708];
    let mut dest: [u8; 8] = [0; 8];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(result, (1, 8));
    assert_eq!(dest, [8, 7, 6, 5, 4, 3, 2, 1]); // Le byte order
}

#[test]
fn test_fill_via_u64_chunks_multiple_chunks() {
    let mut src: [u64; 2] = [0x0102030405060708, 0x090A0B0C0D0E0F10];
    let mut dest: [u8; 16] = [0; 16];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(result, (2, 16));
    assert_eq!(dest, [
        8, 7, 6, 5, 4, 3, 2, 1,
        16, 15, 14, 13, 12, 11, 10, 9 // Le byte order for second u64
    ]);
}

#[test]
fn test_fill_via_u64_chunks_truncated_last_byte() {
    let mut src: [u64; 1] = [0x0102030405060708];
    let mut dest: [u8; 10] = [0; 10];
    let result = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(result, (1, 8));
    assert_eq!(dest, [8, 7, 6, 5, 4, 3, 2, 1]); // Only 8 bytes filled
}

#[test]
fn test_fill_via_u64_chunks_dest_smaller_than_src() {
    let mut src: [u64; 2] = [0x0102030405060708, 0x090A0B0C0D0E0F10];
    let mut dest: [u8; 7] = [0; 7]; // Not enough space to fill all
    let result = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(result, (1, 7)); // Only fill the first u64
    assert_eq!(dest, [8, 7, 6, 5, 4, 3, 2]); // Filled up to 7 bytes
}

#[test]
fn test_fill_via_u64_chunks_full_dest_not_full_src() {
    let mut src: [u64; 1] = [0x0102030405060708];
    let mut dest: [u8; 20] = [0; 20]; // More dest bytes than src bytes available
    let result = fill_via_u64_chunks(&mut src, &mut dest);
    assert_eq!(result, (1, 8));
    assert_eq!(dest[0..8], [8, 7, 6, 5, 4, 3, 2, 1]); // Same as before for filled part
    assert_eq!(dest[8..20], [0; 12]); // Remaining bytes should stay at 0
}

