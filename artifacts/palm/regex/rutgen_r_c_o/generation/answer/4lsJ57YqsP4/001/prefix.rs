// Answer 0

#[test]
fn test_write_literal_class_byte_control_character() {
    let mut output = String::new();
    let writer = &mut Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x00); // Null character
    writer.write_literal_class_byte(0x1F); // Unit separator
}

#[test]
fn test_write_literal_class_byte_whitespace() {
    let mut output = String::new();
    let writer = &mut Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x20); // Space character
}

#[test]
fn test_write_literal_class_byte_non_control_or_whitespace() {
    let mut output = String::new();
    let writer = &mut Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x41); // 'A' character
    writer.write_literal_class_byte(0x7F); // DEL character
}

#[test]
fn test_write_literal_class_byte_high_byte() {
    let mut output = String::new();
    let writer = &mut Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    writer.write_literal_class_byte(0x80); // Beyond ASCII
    writer.write_literal_class_byte(0xFF); // Maximum byte value
}

