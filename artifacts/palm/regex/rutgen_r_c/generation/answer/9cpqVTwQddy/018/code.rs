// Answer 0

#[test]
fn test_fmt_class_ascii_digit() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span { /* initialize with appropriate values */ },
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };

    let result = writer_instance.fmt_class_ascii(&ast);

    assert!(result.is_ok());
    assert_eq!(writer.output, "[:digit:]");
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span { /* initialize with appropriate values */ },
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    let result = writer_instance.fmt_class_ascii(&ast);

    assert!(result.is_ok());
    assert_eq!(writer.output, "[:^digit:]");
}

// Additional cases can be added here to ensure all combinations are tested.

