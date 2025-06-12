// Answer 0

#[test]
fn test_format_i8_positive() {
    let mut buffer = Buffer::new();
    let result = buffer.format(42i8);
    assert_eq!(result, "42");
}

#[test]
fn test_format_i8_negative() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-42i8);
    assert_eq!(result, "-42");
}

#[test]
fn test_format_u8() {
    let mut buffer = Buffer::new();
    let result = buffer.format(200u8);
    assert_eq!(result, "200");
}

#[test]
fn test_format_i16() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-32768i16);
    assert_eq!(result, "-32768");
}

#[test]
fn test_format_u16() {
    let mut buffer = Buffer::new();
    let result = buffer.format(65535u16);
    assert_eq!(result, "65535");
}

#[test]
fn test_format_i32() {
    let mut buffer = Buffer::new();
    let result = buffer.format(2147483647i32);
    assert_eq!(result, "2147483647");
}

#[test]
fn test_format_i64() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-9223372036854775807i64);
    assert_eq!(result, "-9223372036854775807");
}

#[test]
fn test_format_u64() {
    let mut buffer = Buffer::new();
    let result = buffer.format(18446744073709551615u64);
    assert_eq!(result, "18446744073709551615");
}

#[test]
fn test_format_i128() {
    let mut buffer = Buffer::new();
    let result = buffer.format(-170141183460469231731687303715884105728i128);
    assert_eq!(result, "-170141183460469231731687303715884105728");
}

#[test]
fn test_format_u128() {
    let mut buffer = Buffer::new();
    let result = buffer.format(340282366920938463463374607431768211455u128);
    assert_eq!(result, "340282366920938463463374607431768211455");
}

