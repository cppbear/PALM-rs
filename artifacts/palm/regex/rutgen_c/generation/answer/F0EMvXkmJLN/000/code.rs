// Answer 0

#[test]
fn test_is_start_byte_valid_start_bytes() {
    assert!(is_start_byte(TAG_TWO));
    assert!(is_start_byte(TAG_THREE));
    assert!(is_start_byte(TAG_FOUR));
}

#[test]
fn test_is_start_byte_invalid_start_bytes() {
    assert!(!is_start_byte(0b00000000)); // 0
    assert!(!is_start_byte(0b01111111)); // 127
    assert!(!is_start_byte(0b10_000000)); // 128
    assert!(!is_start_byte(0b110_00000)); // 192
} 

#[test]
fn test_is_start_byte_edge_cases() {
    assert!(is_start_byte(0b11000000)); // 192, valid start byte (2 bytes)
    assert!(is_start_byte(0b11100000)); // 224, valid start byte (3 bytes)
    assert!(is_start_byte(0b11110000)); // 240, valid start byte (4 bytes)
    assert!(!is_start_byte(0b10100000)); // 160, invalid (continuation byte)
}

