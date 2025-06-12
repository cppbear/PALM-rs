// Answer 0

#[test]
fn test_write_literal_class_byte_lowercase() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let result = writer.write_literal_class_byte(b'a'); // 'a' is a valid character < 0x7F, non-control, non-whitespace
        assert!(result.is_ok());
        assert_eq!(output, "a"); // Expected output
    }
}

#[test]
fn test_write_literal_class_byte_uppercase() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let result = writer.write_literal_class_byte(b'A'); // 'A' is a valid character < 0x7F, non-control, non-whitespace
        assert!(result.is_ok());
        assert_eq!(output, "A"); // Expected output
    }
}

#[test]
fn test_write_literal_class_byte_whitespace() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let result = writer.write_literal_class_byte(b' '); // space is whitespace
        assert!(result.is_ok());
        assert_eq!(output, "\\x20"); // Expected output in escaped form
    }
}

#[test]
fn test_write_literal_class_byte_control_character() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let result = writer.write_literal_class_byte(b'\n'); // newline is a control character
        assert!(result.is_ok());
        assert_eq!(output, "\\x0A"); // Expected output in escaped form
    }
}

#[test]
fn test_write_literal_class_byte_bound() {
    let mut output = String::new();
    {
        let mut writer = Writer {
            printer: &mut Printer { _priv: () },
            wtr: &mut output,
        };
        let result = writer.write_literal_class_byte(b'\x7F'); // 'DEL' character, valid but control character
        assert!(result.is_ok());
        assert_eq!(output, "\\x7F"); // Expected output in escaped form
    }
}

