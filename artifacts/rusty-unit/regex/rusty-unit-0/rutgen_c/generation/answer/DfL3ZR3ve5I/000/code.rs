// Answer 0

#[test]
fn test_fmt_regular_characters() {
    let unicode_range = ClassUnicodeRange { start: 'a', end: 'z' };
    let mut output = vec![];
    let formatter = &mut fmt::Formatter::new(&mut output);
    unicode_range.fmt(formatter).unwrap();
    let expected_output = "ClassUnicodeRange { start: \"a\", end: \"z\" }";
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_fmt_whitespace_characters() {
    let unicode_range = ClassUnicodeRange { start: ' ', end: '\n' };
    let mut output = vec![];
    let formatter = &mut fmt::Formatter::new(&mut output);
    unicode_range.fmt(formatter).unwrap();
    let expected_output = "ClassUnicodeRange { start: 0x20, end: 0xA }"; // 0x20 is space and 0xA is newline
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_fmt_control_characters() {
    let unicode_range = ClassUnicodeRange { start: '\x00', end: '\x1F' }; // Control characters
    let mut output = vec![];
    let formatter = &mut fmt::Formatter::new(&mut output);
    unicode_range.fmt(formatter).unwrap();
    let expected_output = "ClassUnicodeRange { start: 0x0, end: 0x1F }"; // 0x0 to 0x1F are control characters
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_fmt_range_with_mixed_characters() {
    let unicode_range = ClassUnicodeRange { start: 'a', end: '\x0C' }; // 'a' and Control character
    let mut output = vec![];
    let formatter = &mut fmt::Formatter::new(&mut output);
    unicode_range.fmt(formatter).unwrap();
    let expected_output = "ClassUnicodeRange { start: \"a\", end: 0xC }"; // 0xC is form feed
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

