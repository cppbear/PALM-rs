// Answer 0

#[test]
fn test_format_i8_overflow() {
    let mut buffer = Buffer::new();
    let value: i8 = 128;
    buffer.format(value);
}

#[test]
fn test_format_u8_overflow() {
    let mut buffer = Buffer::new();
    let value: u8 = 256;
    buffer.format(value);
}

#[test]
fn test_format_i16_overflow() {
    let mut buffer = Buffer::new();
    let value: i16 = 32768;
    buffer.format(value);
}

#[test]
fn test_format_u16_overflow() {
    let mut buffer = Buffer::new();
    let value: u16 = 65536;
    buffer.format(value);
}

#[test]
fn test_format_i32_overflow() {
    let mut buffer = Buffer::new();
    let value: i32 = 2147483648;
    buffer.format(value);
}

#[test]
fn test_format_u32_overflow() {
    let mut buffer = Buffer::new();
    let value: u32 = 4294967296;
    buffer.format(value);
}

#[test]
fn test_format_i64_overflow() {
    let mut buffer = Buffer::new();
    let value: i64 = 9223372036854775808;
    buffer.format(value);
}

#[test]
fn test_format_u64_overflow() {
    let mut buffer = Buffer::new();
    let value: u64 = 18446744073709551616;
    buffer.format(value);
}

#[test]
fn test_format_i128_overflow() {
    let mut buffer = Buffer::new();
    let value: i128 = 170141183460469231731687303715884105728;
    buffer.format(value);
}

#[test]
fn test_format_u128_overflow() {
    let mut buffer = Buffer::new();
    let value: u128 = 340282366920938463463374607431768211456;
    buffer.format(value);
}

