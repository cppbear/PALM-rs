// Answer 0

#[test]
fn test_write_literal_byte_control() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x00);
}

#[test]
fn test_write_literal_byte_visible_char() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x41);
}

#[test]
fn test_write_literal_byte_whitespace() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x20);
}

#[test]
fn test_write_literal_byte_non_ascii() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x7F);
}

