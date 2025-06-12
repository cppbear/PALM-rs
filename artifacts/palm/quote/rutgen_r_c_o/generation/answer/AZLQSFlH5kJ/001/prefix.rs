// Answer 0

#[test]
fn test_fmt_bool() {
    let mut formatter = fmt::Formatter::new();
    true.fmt(&mut formatter);
    false.fmt(&mut formatter);
}

#[test]
fn test_fmt_str() {
    let mut formatter = fmt::Formatter::new();
    "".fmt(&mut formatter);
    "a".fmt(&mut formatter);
    "long string".fmt(&mut formatter);
}

#[test]
fn test_fmt_string() {
    let mut formatter = fmt::Formatter::new();
    String::from("empty").fmt(&mut formatter);
    String::from("filled with data").fmt(&mut formatter);
    String::from("another string").fmt(&mut formatter);
}

#[test]
fn test_fmt_char() {
    let mut formatter = fmt::Formatter::new();
    'a'.fmt(&mut formatter);
    'Z'.fmt(&mut formatter);
    '9'.fmt(&mut formatter);
}

#[test]
fn test_fmt_u8() {
    let mut formatter = fmt::Formatter::new();
    0u8.fmt(&mut formatter);
    255u8.fmt(&mut formatter);
}

#[test]
fn test_fmt_u16() {
    let mut formatter = fmt::Formatter::new();
    0u16.fmt(&mut formatter);
    65535u16.fmt(&mut formatter);
}

#[test]
fn test_fmt_u32() {
    let mut formatter = fmt::Formatter::new();
    0u32.fmt(&mut formatter);
    4294967295u32.fmt(&mut formatter);
}

#[test]
fn test_fmt_u64() {
    let mut formatter = fmt::Formatter::new();
    0u64.fmt(&mut formatter);
    18446744073709551615u64.fmt(&mut formatter);
}

#[test]
fn test_fmt_u128() {
    let mut formatter = fmt::Formatter::new();
    0u128.fmt(&mut formatter);
    340282366920938463463374607431768211455u128.fmt(&mut formatter);
}

#[test]
fn test_fmt_usize() {
    let mut formatter = fmt::Formatter::new();
    0usize.fmt(&mut formatter);
    std::usize::MAX.fmt(&mut formatter);
}

