// Answer 0

#[test]
fn test_decode_hex_val_slow_lowercase_a() {
    let input = b'a';
    let expected = Some(input - b'a' + 10);
    assert_eq!(decode_hex_val_slow(input), expected);
}

#[test]
fn test_decode_hex_val_slow_lowercase_b() {
    let input = b'b';
    let expected = Some(input - b'a' + 10);
    assert_eq!(decode_hex_val_slow(input), expected);
}

#[test]
fn test_decode_hex_val_slow_lowercase_c() {
    let input = b'c';
    let expected = Some(input - b'a' + 10);
    assert_eq!(decode_hex_val_slow(input), expected);
}

#[test]
fn test_decode_hex_val_slow_lowercase_d() {
    let input = b'd';
    let expected = Some(input - b'a' + 10);
    assert_eq!(decode_hex_val_slow(input), expected);
}

#[test]
fn test_decode_hex_val_slow_lowercase_e() {
    let input = b'e';
    let expected = Some(input - b'a' + 10);
    assert_eq!(decode_hex_val_slow(input), expected);
}

#[test]
fn test_decode_hex_val_slow_lowercase_f() {
    let input = b'f';
    let expected = Some(input - b'a' + 10);
    assert_eq!(decode_hex_val_slow(input), expected);
}

