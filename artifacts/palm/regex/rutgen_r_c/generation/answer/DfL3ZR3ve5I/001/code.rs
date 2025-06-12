// Answer 0

#[test]
fn test_fmt_with_whitespace_start_and_end() {
    let unicode_range = ClassUnicodeRange {
        start: ' ', // whitespace character
        end: '\n',  // whitespace character
    };

    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| unicode_range.fmt(f));

    assert!(result.is_ok());
    let output_str = String::from_utf8(output).expect("output not valid UTF-8");
    assert!(output_str.contains("start: 0x20")); // ' ' character's code point
    assert!(output_str.contains("end: 0xA"));    // '\n' character's code point
}

#[test]
fn test_fmt_with_control_character_start_and_end() {
    let unicode_range = ClassUnicodeRange {
        start: '\u{0000}', // control character
        end: '\u{001F}',   // control character
    };

    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| unicode_range.fmt(f));

    assert!(result.is_ok());
    let output_str = String::from_utf8(output).expect("output not valid UTF-8");
    assert!(output_str.contains("start: 0x0"));   // '\u{0000}' character's code point
    assert!(output_str.contains("end: 0x1F"));     // '\u{001F}' character's code point
}

