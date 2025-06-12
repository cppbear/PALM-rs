// Answer 0

#[test]
fn test_decode_hex_val_slow_lowercase_a() {
    assert_eq!(decode_hex_val_slow(b'a'), Some(10));
}

#[test]
fn test_decode_hex_val_slow_lowercase_b() {
    assert_eq!(decode_hex_val_slow(b'b'), Some(11));
}

#[test]
fn test_decode_hex_val_slow_lowercase_c() {
    assert_eq!(decode_hex_val_slow(b'c'), Some(12));
}

#[test]
fn test_decode_hex_val_slow_lowercase_d() {
    assert_eq!(decode_hex_val_slow(b'd'), Some(13));
}

#[test]
fn test_decode_hex_val_slow_lowercase_e() {
    assert_eq!(decode_hex_val_slow(b'e'), Some(14));
}

#[test]
fn test_decode_hex_val_slow_lowercase_f() {
    assert_eq!(decode_hex_val_slow(b'f'), Some(15));
}

#[test]
fn test_decode_hex_val_slow_non_hex_char() {
    assert_eq!(decode_hex_val_slow(b'g'), None);
}

