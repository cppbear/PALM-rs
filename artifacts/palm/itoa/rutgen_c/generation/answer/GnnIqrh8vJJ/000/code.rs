// Answer 0

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    assert_eq!(buffer.bytes.len(), i128::MAX_STR_LEN);
}

#[test]
fn test_buffer_format_with_i8() {
    let mut buffer = Buffer::new();
    let result = buffer.format(123_i8);
    assert_eq!(result, "123");
}

#[test]
fn test_buffer_format_with_u8() {
    let mut buffer = Buffer::new();
    let result = buffer.format(255_u8);
    assert_eq!(result, "255");
}

#[test]
fn test_buffer_format_with_i16() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-123_i16);
    assert_eq!(result, "-123");
}

#[test]
fn test_buffer_format_with_u16() {
    let mut buffer = Buffer::new();
    let result = buffer.format(65535_u16);
    assert_eq!(result, "65535");
}

#[test]
fn test_buffer_format_with_i32() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-2147483648_i32);
    assert_eq!(result, "-2147483648");
}

#[test]
fn test_buffer_format_with_u32() {
    let mut buffer = Buffer::new();
    let result = buffer.format(4294967295_u32);
    assert_eq!(result, "4294967295");
}

#[test]
fn test_buffer_format_with_i64() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-9223372036854775808_i64);
    assert_eq!(result, "-9223372036854775808");
}

#[test]
fn test_buffer_format_with_u64() {
    let mut buffer = Buffer::new();
    let result = buffer.format(18446744073709551615_u64);
    assert_eq!(result, "18446744073709551615");
}

#[test]
fn test_buffer_format_with_i128() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-170141183460469231731687303715884105728_i128);
    assert_eq!(result, "-170141183460469231731687303715884105728");
}

#[test]
fn test_buffer_format_with_u128() {
    let mut buffer = Buffer::new();
    let result = buffer.format(340282366920938463463374607431768211455_u128);
    assert_eq!(result, "340282366920938463463374607431768211455");
}

