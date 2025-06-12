// Answer 0

#[test]
fn test_sign_extend_1_byte() {
    assert_eq!(sign_extend(0xFF, 1), -1);
    assert_eq!(sign_extend(0x7F, 1), 127);
}

#[test]
fn test_sign_extend_2_bytes() {
    assert_eq!(sign_extend(0xFFFF, 2), -1);
    assert_eq!(sign_extend(0x7FFF, 2), 32767);
}

#[test]
fn test_sign_extend_3_bytes() {
    assert_eq!(sign_extend(0xFFFFFF, 3), -1);
    assert_eq!(sign_extend(0x7FFFFF, 3), 8388607);
}

#[test]
fn test_sign_extend_4_bytes() {
    assert_eq!(sign_extend(0xFFFFFFFF, 4), -1);
    assert_eq!(sign_extend(0x7FFFFFFF, 4), 2147483647);
}

#[test]
fn test_sign_extend_0_bytes() {
    assert_eq!(sign_extend(0, 0), 0);
}

#[should_panic]
fn test_sign_extend_invalid_nbytes() {
    sign_extend(0xFF, 5);
}

