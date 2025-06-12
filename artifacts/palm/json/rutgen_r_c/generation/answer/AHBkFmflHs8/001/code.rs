// Answer 0

#[test]
fn test_decode_hex_val_slow_digit_0() {
    let result = decode_hex_val_slow(b'0');
    assert_eq!(result, Some(0));
}

#[test]
fn test_decode_hex_val_slow_digit_1() {
    let result = decode_hex_val_slow(b'1');
    assert_eq!(result, Some(1));
}

#[test]
fn test_decode_hex_val_slow_digit_2() {
    let result = decode_hex_val_slow(b'2');
    assert_eq!(result, Some(2));
}

#[test]
fn test_decode_hex_val_slow_digit_3() {
    let result = decode_hex_val_slow(b'3');
    assert_eq!(result, Some(3));
}

#[test]
fn test_decode_hex_val_slow_digit_4() {
    let result = decode_hex_val_slow(b'4');
    assert_eq!(result, Some(4));
}

#[test]
fn test_decode_hex_val_slow_digit_5() {
    let result = decode_hex_val_slow(b'5');
    assert_eq!(result, Some(5));
}

#[test]
fn test_decode_hex_val_slow_digit_6() {
    let result = decode_hex_val_slow(b'6');
    assert_eq!(result, Some(6));
}

#[test]
fn test_decode_hex_val_slow_digit_7() {
    let result = decode_hex_val_slow(b'7');
    assert_eq!(result, Some(7));
}

#[test]
fn test_decode_hex_val_slow_digit_8() {
    let result = decode_hex_val_slow(b'8');
    assert_eq!(result, Some(8));
}

#[test]
fn test_decode_hex_val_slow_digit_9() {
    let result = decode_hex_val_slow(b'9');
    assert_eq!(result, Some(9));
}

