// Answer 0

#[test]
fn test_write_literal_byte_control_character_start() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x00);
}

#[test]
fn test_write_literal_byte_control_character_mid() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x0F);
}

#[test]
fn test_write_literal_byte_control_character_end() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x1F);
}

#[test]
fn test_write_literal_byte_non_printable_character() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_byte(0x7F);
}

