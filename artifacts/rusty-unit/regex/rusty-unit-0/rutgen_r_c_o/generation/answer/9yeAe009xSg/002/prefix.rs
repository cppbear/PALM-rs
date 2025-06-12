// Answer 0

#[test]
fn test_write_literal_char_backslash() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('\\');
}

#[test]
fn test_write_literal_char_dot() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('.');
}

#[test]
fn test_write_literal_char_plus() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('+');
}

#[test]
fn test_write_literal_char_star() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('*');
}

#[test]
fn test_write_literal_char_question() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('?');
}

#[test]
fn test_write_literal_char_open_parenthesis() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('(');
}

#[test]
fn test_write_literal_char_close_parenthesis() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char(')');
}

#[test]
fn test_write_literal_char_pipe() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('|');
}

#[test]
fn test_write_literal_char_open_bracket() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('[');
}

#[test]
fn test_write_literal_char_close_bracket() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char(']');
}

#[test]
fn test_write_literal_char_open_brace() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('{');
}

#[test]
fn test_write_literal_char_close_brace() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('}');
}

#[test]
fn test_write_literal_char_caret() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('^');
}

#[test]
fn test_write_literal_char_dollar() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('$');
}

#[test]
fn test_write_literal_char_hash() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('#');
}

#[test]
fn test_write_literal_char_ampersand() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('&');
}

#[test]
fn test_write_literal_char_dash() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('-');
}

#[test]
fn test_write_literal_char_tilde() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.write_literal_char('~');
}

