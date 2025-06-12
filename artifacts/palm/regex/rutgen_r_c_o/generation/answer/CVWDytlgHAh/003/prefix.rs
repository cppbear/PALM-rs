// Answer 0

#[test]
fn test_write_literal_byte_valid_range_1() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x21); // '!'
}

#[test]
fn test_write_literal_byte_valid_range_2() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x7E); // '~'
}

#[test]
fn test_write_literal_byte_invalid_control() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x00); // NUL character (control)
}

#[test]
fn test_write_literal_byte_invalid_whitespace() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x0A); // LF character (whitespace)
}

#[test]
fn test_write_literal_byte_above_ascii() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x80); // Out of the valid range
}

