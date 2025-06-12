// Answer 0

fn test_fmt_class_ascii_lower() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let class_ascii = ast::ClassAscii {
        span: Span { start: 0, end: 10 },
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    };

    writer.fmt_class_ascii(&class_ascii).unwrap();
    assert_eq!(mock_writer.output, "[:lower:]");
}

fn test_fmt_class_ascii_lower_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let class_ascii = ast::ClassAscii {
        span: Span { start: 0, end: 10 },
        kind: ast::ClassAsciiKind::Lower,
        negated: true,
    };

    writer.fmt_class_ascii(&class_ascii).unwrap();
    assert_eq!(mock_writer.output, "[:^lower:]");
}

fn test_fmt_class_ascii_invalid_negated() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let class_ascii = ast::ClassAscii {
        span: Span { start: 0, end: 10 },
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    };

    writer.fmt_class_ascii(&class_ascii).unwrap();
    assert_eq!(mock_writer.output, "[:lower:]");
}

