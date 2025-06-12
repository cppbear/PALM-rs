// Answer 0

#[test]
fn test_decode_hex_val_slow_with_digit_0() {
    assert_eq!(decode_hex_val_slow(b'0'), Some(0));
}

#[test]
fn test_decode_hex_val_slow_with_digit_9() {
    assert_eq!(decode_hex_val_slow(b'9'), Some(9));
}

#[test]
fn test_decode_hex_val_slow_with_uppercase_a() {
    assert_eq!(decode_hex_val_slow(b'A'), Some(10));
}

#[test]
fn test_decode_hex_val_slow_with_uppercase_f() {
    assert_eq!(decode_hex_val_slow(b'F'), Some(15));
}

#[test]
fn test_decode_hex_val_slow_with_lowercase_a() {
    assert_eq!(decode_hex_val_slow(b'a'), Some(10));
}

#[test]
fn test_decode_hex_val_slow_with_lowercase_f() {
    assert_eq!(decode_hex_val_slow(b'f'), Some(15));
}

#[test]
fn test_decode_hex_val_slow_with_non_hex_character() {
    assert_eq!(decode_hex_val_slow(b'G'), None);
}

#[test]
fn test_decode_hex_val_slow_with_special_character() {
    assert_eq!(decode_hex_val_slow(b'!'), None);
}

#[test]
fn test_decode_hex_val_slow_with_negative_value() {
    assert_eq!(decode_hex_val_slow(0xFF), None);
}

