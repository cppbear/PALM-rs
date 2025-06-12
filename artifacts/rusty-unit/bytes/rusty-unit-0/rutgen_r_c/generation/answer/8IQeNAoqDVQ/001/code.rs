// Answer 0

#[test]
fn test_offset_from_positive_offset() {
    let original = 0x1000 as *const u8;
    let dst = 0x1010 as *const u8;
    let result = offset_from(dst, original);
    assert_eq!(result, 16);
}

#[test]
fn test_offset_from_zero_offset() {
    let original = 0x2000 as *const u8;
    let dst = 0x2000 as *const u8;
    let result = offset_from(dst, original);
    assert_eq!(result, 0);
}

#[test]
fn test_offset_from_large_offset() {
    let original = 0x3000 as *const u8;
    let dst = 0x4000 as *const u8;
    let result = offset_from(dst, original);
    assert_eq!(result, 4096);
}

#[should_panic]
fn test_offset_from_negative_offset() {
    let original = 0x5000 as *const u8;
    let dst = 0x4000 as *const u8; // This should trigger a panic because dst < original
    let _result = offset_from(dst, original);
}

