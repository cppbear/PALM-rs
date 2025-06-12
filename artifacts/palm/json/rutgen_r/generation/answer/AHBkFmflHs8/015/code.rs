// Answer 0

#[test]
fn test_decode_hex_val_slow_with_non_hex_character() {
    assert_eq!(decode_hex_val_slow(b'G'), None);
}

#[test]
fn test_decode_hex_val_slow_with_special_character() {
    assert_eq!(decode_hex_val_slow(b'@'), None);
}

#[test]
fn test_decode_hex_val_slow_with_lowercase_out_of_range() {
    assert_eq!(decode_hex_val_slow(b'g'), None);
}

#[test]
fn test_decode_hex_val_slow_with_non_hex_ascii_character() {
    assert_eq!(decode_hex_val_slow(b' '), None);
}

#[test]
fn test_decode_hex_val_slow_with_high_value() {
    assert_eq!(decode_hex_val_slow(255), None);
}

