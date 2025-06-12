// Answer 0

#[test]
fn test_fill_via_u32_chunks_partial_fill() {
    let mut src: [u32; 2] = [0xDEADBEEF, 0xBEEFBEEF];
    let mut dest: [u8; 5] = [0; 5];
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(&mut src, &mut dest);
    assert_eq!(consumed_u32, 1);
    assert_eq!(filled_u8, 4);
    assert_eq!(dest, [0xEF, 0xBE, 0xAD, 0xDE, 0]);
}

#[test]
fn test_fill_via_u32_chunks_exact_fill() {
    let mut src: [u32; 2] = [0xDEADBEEF, 0xBEEFBEEF];
    let mut dest: [u8; 8] = [0; 8];
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(&mut src, &mut dest);
    assert_eq!(consumed_u32, 2);
    assert_eq!(filled_u8, 8);
    assert_eq!(dest, [0xEF, 0xBE, 0xAD, 0xDE, 0xEF, 0xBE, 0xBE, 0]);
}

#[test]
fn test_fill_via_u32_chunks_no_fill() {
    let mut src: [u32; 0] = [];
    let mut dest: [u8; 4] = [0; 4];
    let (consumed_u32, filled_u8) = fill_via_u32_chunks(&mut src, &mut dest);
    assert_eq!(consumed_u32, 0);
    assert_eq!(filled_u8, 0);
    assert_eq!(dest, [0; 4]);
}

