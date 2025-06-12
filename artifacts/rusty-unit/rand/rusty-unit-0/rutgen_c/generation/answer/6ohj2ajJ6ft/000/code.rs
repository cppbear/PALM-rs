// Answer 0

#[test]
fn test_fill_via_u32_chunks_exact_fill() {
    let mut src: [u32; 3] = [1, 2, 3];
    let mut dest: [u8; 12] = [0; 12];
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 3);
    assert_eq!(filled, 12);
    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn test_fill_via_u32_chunks_partial_fill() {
    let mut src: [u32; 2] = [1, 2];
    let mut dest: [u8; 5] = [0; 5];
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 2);
    assert_eq!(filled, 5);
    assert_eq!(&dest, &[1, 0, 0, 0, 2]);
}

#[test]
fn test_fill_via_u32_chunks_zero_length_dest() {
    let mut src: [u32; 3] = [1, 2, 3];
    let mut dest: [u8; 0] = [];
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 0);
    assert_eq!(filled, 0);
}

#[test]
fn test_fill_via_u32_chunks_dest_smaller_than_u32() {
    let mut src: [u32; 1] = [1];
    let mut dest: [u8; 2] = [0; 2];
    let (consumed, filled) = fill_via_u32_chunks(&mut src, &mut dest);
    assert_eq!(consumed, 1);
    assert_eq!(filled, 2);
    assert_eq!(&dest, &[1, 0]);
}

