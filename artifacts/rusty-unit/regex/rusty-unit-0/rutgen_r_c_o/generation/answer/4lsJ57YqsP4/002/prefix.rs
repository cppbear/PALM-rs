// Answer 0

#[test]
fn test_write_literal_class_byte_lower_bound_non_control_non_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x21).ok(); // '!' character
}

#[test]
fn test_write_literal_class_byte_upper_bound_non_control_non_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x7E).ok(); // '~' character
}

#[test]
fn test_write_literal_class_byte_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x20).ok(); // whitespace character ' '
}

#[test]
fn test_write_literal_class_byte_control_character() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x0A).ok(); // control character '\n'
}

#[test]
fn test_write_literal_class_byte_invalid_character() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0xFF).ok(); // Expect \xFF
}

