// Answer 0

#[test]
fn test_write_literal_class_byte_valid_input_33() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(33);
}

#[test]
fn test_write_literal_class_byte_valid_input_64() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(64);
}

#[test]
fn test_write_literal_class_byte_valid_input_90() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(90);
}

#[test]
fn test_write_literal_class_byte_valid_input_126() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(126);
} 

#[test]
fn test_write_literal_class_byte_invalid_input_0() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(0);
}

#[test]
fn test_write_literal_class_byte_invalid_input_7() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(7);
}

#[test]
fn test_write_literal_class_byte_invalid_input_9() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(9);
}

#[test]
fn test_write_literal_class_byte_invalid_input_10() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(10);
}

#[test]
fn test_write_literal_class_byte_invalid_input_13() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(13);
} 

#[test]
fn test_write_literal_class_byte_invalid_input_128() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_class_byte(128);
}

