// Answer 0

#[test]
fn test_format_i8_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(-128i8);
}

#[test]
fn test_format_i8_zero() {
    let mut buffer = Buffer::new();
    buffer.format(0i8);
}

#[test]
fn test_format_i8_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(127i8);
}

#[test]
fn test_format_u8_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(0u8);
}

#[test]
fn test_format_u8_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(255u8);
}

#[test]
fn test_format_i16_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(-32768i16);
}

#[test]
fn test_format_i16_zero() {
    let mut buffer = Buffer::new();
    buffer.format(0i16);
}

#[test]
fn test_format_i16_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(32767i16);
}

#[test]
fn test_format_u16_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(0u16);
}

#[test]
fn test_format_u16_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(65535u16);
}

#[test]
fn test_format_i32_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(-2147483648i32);
}

#[test]
fn test_format_i32_zero() {
    let mut buffer = Buffer::new();
    buffer.format(0i32);
}

#[test]
fn test_format_i32_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(2147483647i32);
}

#[test]
fn test_format_u32_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(0u32);
}

#[test]
fn test_format_u32_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(4294967295u32);
}

#[test]
fn test_format_i64_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(-9223372036854775808i64);
}

#[test]
fn test_format_i64_zero() {
    let mut buffer = Buffer::new();
    buffer.format(0i64);
}

#[test]
fn test_format_i64_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(9223372036854775807i64);
}

#[test]
fn test_format_u64_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(0u64);
}

#[test]
fn test_format_u64_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(18446744073709551615u64);
}

#[test]
fn test_format_i128_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(-170141183460469231731687303715884105728i128);
}

#[test]
fn test_format_i128_zero() {
    let mut buffer = Buffer::new();
    buffer.format(0i128);
}

#[test]
fn test_format_i128_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(170141183460469231731687303715884105727i128);
}

#[test]
fn test_format_u128_minimum() {
    let mut buffer = Buffer::new();
    buffer.format(0u128);
}

#[test]
fn test_format_u128_maximum() {
    let mut buffer = Buffer::new();
    buffer.format(340282366920938463463374607431768211456u128);
}

