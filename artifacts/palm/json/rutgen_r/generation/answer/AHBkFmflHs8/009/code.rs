// Answer 0

#[test]
fn test_decode_hex_val_slow_uppercase_a() {
    let result = decode_hex_val_slow(b'A');
    assert_eq!(result, Some(10));
}

#[test]
fn test_decode_hex_val_slow_uppercase_b() {
    let result = decode_hex_val_slow(b'B');
    assert_eq!(result, Some(11));
}

#[test]
fn test_decode_hex_val_slow_uppercase_c() {
    let result = decode_hex_val_slow(b'C');
    assert_eq!(result, Some(12));
}

#[test]
fn test_decode_hex_val_slow_uppercase_d() {
    let result = decode_hex_val_slow(b'D');
    assert_eq!(result, Some(13));
}

#[test]
fn test_decode_hex_val_slow_uppercase_e() {
    let result = decode_hex_val_slow(b'E');
    assert_eq!(result, Some(14));
}

#[test]
fn test_decode_hex_val_slow_uppercase_f() {
    let result = decode_hex_val_slow(b'F');
    assert_eq!(result, Some(15));
}

#[test]
fn test_decode_hex_val_slow_not_hex() {
    let result = decode_hex_val_slow(b'G');
    assert_eq!(result, None);
}

