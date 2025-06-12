// Answer 0

#[test]
fn test_format_i8_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i8::MAX);
    assert_eq!(output.len(), 4);
    assert_eq!(output, "127");
}

#[test]
fn test_format_i8_min_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i8::MIN);
    assert_eq!(output.len(), 4);
    assert_eq!(output, "-128");
}

#[test]
fn test_format_u8_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(u8::MAX);
    assert_eq!(output.len(), 3);
    assert_eq!(output, "255");
}

#[test]
fn test_format_i16_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i16::MAX);
    assert_eq!(output.len(), 6);
    assert_eq!(output, "32767");
}

#[test]
fn test_format_i16_min_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i16::MIN);
    assert_eq!(output.len(), 6);
    assert_eq!(output, "-32768");
}

#[test]
fn test_format_u16_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(u16::MAX);
    assert_eq!(output.len(), 5);
    assert_eq!(output, "65535");
}

#[test]
fn test_format_i32_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i32::MAX);
    assert_eq!(output.len(), 11);
    assert_eq!(output, "2147483647");
}

#[test]
fn test_format_i32_min_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i32::MIN);
    assert_eq!(output.len(), 11);
    assert_eq!(output, "-2147483648");
}

#[test]
fn test_format_u32_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(u32::MAX);
    assert_eq!(output.len(), 10);
    assert_eq!(output, "4294967295");
}

#[test]
fn test_format_i64_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i64::MAX);
    assert_eq!(output.len(), 20);
    assert_eq!(output, "9223372036854775807");
}

#[test]
fn test_format_i64_min_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i64::MIN);
    assert_eq!(output.len(), 20);
    assert_eq!(output, "-9223372036854775808");
}

#[test]
fn test_format_u64_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(u64::MAX);
    assert_eq!(output.len(), 20);
    assert_eq!(output, "18446744073709551615");
}

#[test]
fn test_format_i128_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i128::MAX);
    assert_eq!(output.len(), 40);
    assert_eq!(output, "170141183460469231731687303715884105727");
}

#[test]
fn test_format_i128_min_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(i128::MIN);
    assert_eq!(output.len(), 40);
    assert_eq!(output, "-170141183460469231731687303715884105728");
}

#[test]
fn test_format_u128_max_length() {
    let mut buffer = Buffer::new();
    let output = buffer.format(u128::MAX);
    assert_eq!(output.len(), 39);
    assert_eq!(output, "340282366920938463463374607431768211455");
}

