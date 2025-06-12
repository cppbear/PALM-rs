// Answer 0

#[test]
fn test_fill_via_u32_chunks_full_fill() {
    let mut src: [u32; 2] = [0x01020304, 0x05060708];
    let mut dest: [u8; 8] = [0; 8];
    
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    
    assert_eq!(consumed, 2);
    assert_eq!(filled, 8);
    assert_eq!(dest, [0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05]);
}

#[test]
fn test_fill_via_u32_chunks_partial_fill() {
    let mut src: [u32; 2] = [0x01020304, 0x05060708];
    let mut dest: [u8; 6] = [0; 6];
    
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    
    assert_eq!(consumed, 2);
    assert_eq!(filled, 6);
    assert_eq!(dest, [0x04, 0x03, 0x02, 0x01, 0x08, 0x07]);
}

#[test]
fn test_fill_via_u32_chunks_empty_src() {
    let mut src: [u32; 0] = [];
    let mut dest: [u8; 4] = [0; 4];
    
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    
    assert_eq!(consumed, 0);
    assert_eq!(filled, 0);
    assert_eq!(dest, [0; 4]);
}

#[test]
fn test_fill_via_u32_chunks_empty_dest() {
    let mut src: [u32; 2] = [0x01020304, 0x05060708];
    let mut dest: [u8; 0] = [];
    
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    
    assert_eq!(consumed, 0);
    assert_eq!(filled, 0);
}

#[test]
fn test_fill_via_u32_chunks_single_word() {
    let mut src: [u32; 1] = [0x01020304];
    let mut dest: [u8; 4] = [0; 4];
    
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    
    assert_eq!(consumed, 1);
    assert_eq!(filled, 4);
    assert_eq!(dest, [0x04, 0x03, 0x02, 0x01]);
}

