// Answer 0

#[test]
fn test_write_literal_char_non_meta() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let result = writer.write_literal_char('a');
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "a");
}

#[test]
fn test_write_literal_char_meta() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let result = writer.write_literal_char('.');
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\.");
}

#[test]
fn test_write_literal_char_escape_sequence() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let result = writer.write_literal_char('\\');
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "\\\\");
}

