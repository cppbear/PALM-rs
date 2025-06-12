// Answer 0

#[test]
fn test_write_literal_class_byte_above_maximum() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let b = 0x80; // b > 0x7F
    let _ = writer.write_literal_class_byte(b);
}

#[test]
fn test_write_literal_class_byte_minimum() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let b = 0x00; // b < 0x00, testing the lowest byte value
    let _ = writer.write_literal_class_byte(b);
}

