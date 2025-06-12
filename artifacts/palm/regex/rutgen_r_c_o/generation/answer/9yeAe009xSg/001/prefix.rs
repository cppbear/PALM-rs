// Answer 0

#[test]
fn test_write_literal_char_with_meta_character() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let _ = writer.write_literal_char('\\');
    let _ = writer.write_literal_char('.');
    let _ = writer.write_literal_char('+');
    let _ = writer.write_literal_char('*');
    let _ = writer.write_literal_char('?');
    let _ = writer.write_literal_char('(');
    let _ = writer.write_literal_char(')');
    let _ = writer.write_literal_char('|');
    let _ = writer.write_literal_char('[');
    let _ = writer.write_literal_char(']');
    let _ = writer.write_literal_char('{');
    let _ = writer.write_literal_char('}');
    let _ = writer.write_literal_char('^');
    let _ = writer.write_literal_char('$');
    let _ = writer.write_literal_char('#');
    let _ = writer.write_literal_char('&');
    let _ = writer.write_literal_char('-');
    let _ = writer.write_literal_char('~');
}

#[test]
#[should_panic]
fn test_write_literal_char_with_write_str_err() {
    struct ErrWriter;

    impl fmt::Write for ErrWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Err(fmt::Error) // simulate an error on write_str
        }
    }

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: ErrWriter };

    let _ = writer.write_literal_char('.');
}

