// Answer 0

#[test]
fn test_write_literal_byte_control_character() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let result = writer.write_literal_byte(0x00); // Control character
    assert!(result.is_ok());
    assert_eq!(output, "(?-u:\\x00)"); // Expected output for control character
}

#[test]
fn test_write_literal_byte_whitespace_character() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let result = writer.write_literal_byte(0x20); // Whitespace character (space)
    assert!(result.is_ok());
    assert_eq!(output, "(?-u:\\x20)"); // Expected output for whitespace character
}

#[test]
fn test_write_literal_byte_high_ascii() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let result = writer.write_literal_byte(0x80); // High ASCII value
    assert!(result.is_ok());
    assert_eq!(output, "(?-u:\\x80)"); // Expected output for byte > 0x7F
}

#[test]
fn test_write_literal_byte_non_whitespace_non_control() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let result = writer.write_literal_byte(0x41); // ASCII character 'A'
    assert!(result.is_ok());
    assert_eq!(output, "A"); // Expected output for non-control, non-whitespace character
}

