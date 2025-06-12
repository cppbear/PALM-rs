// Answer 0

#[test]
fn test_offset_from_positive_difference() {
    let original: *const u8 = &10u8;
    let dst: *const u8 = &20u8;
    let result = offset_from(dst, original);
    assert_eq!(result, 10);
}

#[test]
fn test_offset_from_zero_difference() {
    let original: *const u8 = &15u8;
    let dst: *const u8 = &15u8;
    let result = offset_from(dst, original);
    assert_eq!(result, 0);
}

#[test]
fn test_offset_from_large_difference() {
    let original: *const u8 = 0x1_0000_0000 as *const u8; // Arbitrarily large address
    let dst: *const u8 = 0x1_0000_000A as *const u8; // 10 bytes later
    let result = offset_from(dst, original);
    assert_eq!(result, 10);
}

#[test]
fn test_offset_from_boundary_condition() {
    let original: *const u8 = 0 as *const u8; // Starting address
    let dst: *const u8 = 100 as *const u8; // 100 bytes away
    let result = offset_from(dst, original);
    assert_eq!(result, 100);
}

