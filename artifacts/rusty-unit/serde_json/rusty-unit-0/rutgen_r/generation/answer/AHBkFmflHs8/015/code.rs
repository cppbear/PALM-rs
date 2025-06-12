// Answer 0

#[test]
fn test_decode_hex_val_slow_out_of_range() {
    let val: u8 = b'G';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, None);
}

#[test]
fn test_decode_hex_val_slow_special_character() {
    let val: u8 = b'!';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, None);
}

#[test]
fn test_decode_hex_val_slow_negative() {
    let val: u8 = 255; // Out of expected range
    let result = decode_hex_val_slow(val);
    assert_eq!(result, None);
}

#[test]
fn test_decode_hex_val_slow_non_hex_digit() {
    let val: u8 = b'z'; // Lowercase non-hex character
    let result = decode_hex_val_slow(val);
    assert_eq!(result, None);
}

#[test]
fn test_decode_hex_val_slow_control_character() {
    let val: u8 = 0; // Non-printable control character
    let result = decode_hex_val_slow(val);
    assert_eq!(result, None);
}

