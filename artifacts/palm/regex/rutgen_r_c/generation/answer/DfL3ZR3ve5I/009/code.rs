// Answer 0

#[test]
fn test_fmt_with_normal_characters() {
    let range = ClassUnicodeRange {
        start: 'a',
        end: 'z',
    };
    let mut output = String::new();
    let result = range.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "ClassUnicodeRange { start: \"a\", end: \"z\" }");
}

#[test]
fn test_fmt_with_characters_with_control() {
    let range = ClassUnicodeRange {
        start: '\u{7F}', // DEL control character
        end: 'z',
    };
    let mut output = String::new();
    let result = range.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "ClassUnicodeRange { start: \"0x7F\", end: \"z\" }");
}

#[test]
fn test_fmt_with_whitespace_characters() {
    let range = ClassUnicodeRange {
        start: ' ',
        end: 'z',
    };
    let mut output = String::new();
    let result = range.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "ClassUnicodeRange { start: \"0x20\", end: \"z\" }");
}

#[test]
fn test_fmt_with_both_control_characters() {
    let range = ClassUnicodeRange {
        start: '\u{0}', // NUL control character
        end: '\u{8F}', // control character
    };
    let mut output = String::new();
    let result = range.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "ClassUnicodeRange { start: \"0x0\", end: \"0x8F\" }");
}

