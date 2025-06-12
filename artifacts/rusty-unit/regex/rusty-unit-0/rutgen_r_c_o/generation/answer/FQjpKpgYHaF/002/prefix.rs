// Answer 0

#[test]
fn test_char_fmt_panic_case_lower_bound() {
    let char_instance = Char(0);
    let _ = format!("{:?}", char_instance);
}

#[test]
fn test_char_fmt_panic_case_upper_bound() {
    let char_instance = Char(0x110000);
    let _ = format!("{:?}", char_instance);
}

#[test]
fn test_char_fmt_panic_case_above_upper_bound() {
    let char_instance = Char(u32::MAX);
    let _ = format!("{:?}", char_instance);
}

