// Answer 0

#[test]
fn test_decode_hex_val_slow_a() {
    let val: u8 = b'a';
    assert_eq!(decode_hex_val_slow(val), Some(val - b'a' + 10));
}

#[test]
fn test_decode_hex_val_slow_b() {
    let val: u8 = b'b';
    assert_eq!(decode_hex_val_slow(val), Some(val - b'a' + 10));
}

#[test]
fn test_decode_hex_val_slow_c() {
    let val: u8 = b'c';
    assert_eq!(decode_hex_val_slow(val), Some(val - b'a' + 10));
}

#[test]
fn test_decode_hex_val_slow_d() {
    let val: u8 = b'd';
    assert_eq!(decode_hex_val_slow(val), Some(val - b'a' + 10));
}

#[test]
fn test_decode_hex_val_slow_e() {
    let val: u8 = b'e';
    assert_eq!(decode_hex_val_slow(val), Some(val - b'a' + 10));
}

#[test]
fn test_decode_hex_val_slow_f() {
    let val: u8 = b'f';
    assert_eq!(decode_hex_val_slow(val), Some(val - b'a' + 10));
}

#[test]
fn test_decode_hex_val_slow_invalid() {
    let val: u8 = b'G';
    assert_eq!(decode_hex_val_slow(val), None);
}

#[test]
fn test_decode_hex_val_slow_non_hex() {
    let val: u8 = b'!';
    assert_eq!(decode_hex_val_slow(val), None);
}

