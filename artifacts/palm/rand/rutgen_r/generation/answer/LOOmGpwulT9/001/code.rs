// Answer 0

#[test]
fn test_fill_via_u64_chunks() {
    let mut src: [u64; 3] = [0x0123456789ABCDEF, 0xFEDCBA9876543210, 0xA1B2C3D4E5F60708];
    let mut dest: [u8; 24] = [0; 24];

    let (consumed_u64, filled_u8) = fill_via_u64_chunks(&mut src, &mut dest);
    
    assert_eq!(consumed_u64, 3); // 3 words of u64 consumed
    assert!(filled_u8 <= dest.len()); // should not exceed destination length
    assert_eq!(filled_u8, 24); // all 24 bytes should be filled

    let expected_dest: [u8; 24] = [
        0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01,
        0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE,
        0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
    ];
    assert_eq!(dest, expected_dest);
}

#[test]
fn test_fill_via_u64_chunks_partial_fill() {
    let mut src: [u64; 2] = [0x1234567890ABCDEF, 0x0FEDCBA98765432];
    let mut dest: [u8; 10] = [0; 10];

    let (consumed_u64, filled_u8) = fill_via_u64_chunks(&mut src, &mut dest);

    assert_eq!(consumed_u64, 2); // 2 words of u64 consumed
    assert!(filled_u8 <= dest.len()); // should not exceed destination length
    assert_eq!(filled_u8, 10); // all 10 bytes should be filled

    let expected_dest: [u8; 10] = [
        0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12,
        0x32, 0x54, // Only filling part of the second word
    ];
    assert_eq!(&dest[0..8], &expected_dest[0..8]);
}

#[test]
fn test_fill_via_u64_chunks_empty_dest() {
    let mut src: [u64; 5] = [0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x1234567890ABCDEF, 0x0FEDCBA98765432, 0x0123456789ABCDEF];
    let mut dest: [u8; 0] = [];

    let (consumed_u64, filled_u8) = fill_via_u64_chunks(&mut src, &mut dest);

    assert_eq!(consumed_u64, 0); // No words should be consumed
    assert_eq!(filled_u8, 0); // No bytes should be filled
}

#[test]
#[should_panic]
fn test_fill_via_u64_chunks_insufficient_bytes() {
    let mut src: [u64; 1] = [0x0011223344556677];
    let mut dest: [u8; 7] = [0; 7]; // Not enough capacity for complete fill

    let _ = fill_via_u64_chunks(&mut src, &mut dest); // Should not panic, but may not fill up
}

