// Answer 0

#[test]
fn test_fmt_with_normal_chars() {
    let range = ClassUnicodeRange { start: 'A', end: 'Z' };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    range.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_with_special_chars() {
    let range = ClassUnicodeRange { start: '#', end: '%' };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    range.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_with_lowercase_letters() {
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    range.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_with_number_range() {
    let range = ClassUnicodeRange { start: '1', end: '9' };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    range.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_with_non_whitespace_non_control_edge_values() {
    let range = ClassUnicodeRange { start: '!', end: '~' };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    range.fmt(formatter).unwrap();
}

