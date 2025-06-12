// Answer 0

#[test]
fn test_write_literal_char_lowercase() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('a');
}

#[test]
fn test_write_literal_char_uppercase() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('Z');
}

#[test]
fn test_write_literal_char_digit() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('5');
}

#[test]
fn test_write_literal_char_space() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char(' ');
}

#[test]
fn test_write_literal_char_special_char1() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('!');
}

#[test]
fn test_write_literal_char_special_char2() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('\'');
}

#[test]
fn test_write_literal_char_special_char3() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('“');
}

#[test]
fn test_write_literal_char_special_char4() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('·');
}

#[test]
fn test_write_literal_char_special_char5() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('‘');
}

#[test]
fn test_write_literal_char_special_char6() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('¿');
}

#[test]
fn test_write_literal_char_extended_ascii() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('À');
} 

#[test]
fn test_write_literal_char_extended_ascii_upper() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('ÿ');
}

