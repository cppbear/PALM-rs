// Answer 0

#[test]
fn test_write_literal_byte_above_7f() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.write_literal_byte(128);
    writer.write_literal_byte(200);
    writer.write_literal_byte(255);
}

#[test]
fn test_write_literal_byte_boundary() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    writer.write_literal_byte(255);
}

