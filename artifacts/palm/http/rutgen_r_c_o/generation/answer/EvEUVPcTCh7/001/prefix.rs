// Answer 0

#[test]
fn test_fmt_u16() {
    let val: u16 = 0;
    let header_value = HeaderValue::from(val);
}

#[test]
fn test_fmt_i16() {
    let val: i16 = -32768;
    let header_value = HeaderValue::from(val);
}

#[test]
fn test_fmt_u32() {
    let val: u32 = 4294967295;
    let header_value = HeaderValue::from(val);
}

#[test]
fn test_fmt_i32() {
    let val: i32 = -2147483648;
    let header_value = HeaderValue::from(val);
}

#[test]
fn test_fmt_u64() {
    let val: u64 = 18446744073709551615;
    let header_value = HeaderValue::from(val);
}

#[test]
fn test_fmt_i64() {
    let val: i64 = -9223372036854775808;
    let header_value = HeaderValue::from(val);
}

#[test]
fn test_fmt_usize() {
    let val: usize = 0;
    let header_value = HeaderValue::from(val);
}

#[test]
fn test_fmt_isize() {
    let val: isize = -9223372036854775808;
    let header_value = HeaderValue::from(val);
}

