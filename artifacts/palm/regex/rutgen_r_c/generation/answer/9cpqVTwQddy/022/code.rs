// Answer 0

#[test]
fn test_fmt_class_ascii_blank_not_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::default(), // assuming a default implementation for Span
        kind: ast::ClassAsciiKind::Blank,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:blank:]");
}

#[test]
fn test_fmt_class_ascii_blank_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::default(), // assuming a default implementation for Span
        kind: ast::ClassAsciiKind::Blank,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^blank:]");
}

#[test]
fn test_fmt_class_ascii_digit_not_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::default(), // assuming a default implementation for Span
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:digit:]");
}

#[test]
fn test_fmt_class_ascii_asci_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span::default(), // assuming a default implementation for Span
        kind: ast::ClassAsciiKind::Ascii,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^ascii:]");
}

