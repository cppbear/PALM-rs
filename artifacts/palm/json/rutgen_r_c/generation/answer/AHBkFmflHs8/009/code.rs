// Answer 0

#[test]
fn test_decode_hex_val_slow_a() {
    let val = b'A'; // matches b'A'..=b'F'
    assert_eq!(decode_hex_val_slow(val), Some(val - b'A' + 10)); // expected Some(10)
}

#[test]
fn test_decode_hex_val_slow_b() {
    let val = b'B'; // matches b'A'..=b'F'
    assert_eq!(decode_hex_val_slow(val), Some(val - b'A' + 10)); // expected Some(11)
}

#[test]
fn test_decode_hex_val_slow_c() {
    let val = b'C'; // matches b'A'..=b'F'
    assert_eq!(decode_hex_val_slow(val), Some(val - b'A' + 10)); // expected Some(12)
}

#[test]
fn test_decode_hex_val_slow_d() {
    let val = b'D'; // matches b'A'..=b'F'
    assert_eq!(decode_hex_val_slow(val), Some(val - b'A' + 10)); // expected Some(13)
}

#[test]
fn test_decode_hex_val_slow_e() {
    let val = b'E'; // matches b'A'..=b'F'
    assert_eq!(decode_hex_val_slow(val), Some(val - b'A' + 10)); // expected Some(14)
}

#[test]
fn test_decode_hex_val_slow_f() {
    let val = b'F'; // matches b'A'..=b'F'
    assert_eq!(decode_hex_val_slow(val), Some(val - b'A' + 10)); // expected Some(15)
}

