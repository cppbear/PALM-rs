// Answer 0

#[test]
fn test_decode_hex_val_slow_uppercase_a() {
    let val = b'A';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, Some(10));
}

#[test]
fn test_decode_hex_val_slow_uppercase_b() {
    let val = b'B';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, Some(11));
}

#[test]
fn test_decode_hex_val_slow_uppercase_c() {
    let val = b'C';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, Some(12));
}

#[test]
fn test_decode_hex_val_slow_uppercase_d() {
    let val = b'D';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, Some(13));
}

#[test]
fn test_decode_hex_val_slow_uppercase_e() {
    let val = b'E';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, Some(14));
}

#[test]
fn test_decode_hex_val_slow_uppercase_f() {
    let val = b'F';
    let result = decode_hex_val_slow(val);
    assert_eq!(result, Some(15));
}

