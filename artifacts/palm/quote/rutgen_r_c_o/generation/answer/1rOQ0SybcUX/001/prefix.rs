// Answer 0

#[test]
fn test_fmt_bool() {
    let value_true: &bool = &true;
    let value_false: &bool = &false;
    let mut formatter = fmt::Formatter::default();
    value_true.fmt(&mut formatter);
    value_false.fmt(&mut formatter);
}

#[test]
fn test_fmt_str() {
    let value: &str = "test string";
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_string() {
    let value: &String = &String::from("test string");
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_char() {
    let value: &char = &'a';
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_u8() {
    let value: &u8 = &255;
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_u16() {
    let value: &u16 = &65535;
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_u32() {
    let value: &u32 = &4294967295;
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_u64() {
    let value: &u64 = &18446744073709551615;
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_u128() {
    let value: &u128 = &340282366920938463463374607431768211455;
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_usize() {
    let value: &usize = &std::usize::MAX;
    let mut formatter = fmt::Formatter::default();
    value.fmt(&mut formatter);
}

