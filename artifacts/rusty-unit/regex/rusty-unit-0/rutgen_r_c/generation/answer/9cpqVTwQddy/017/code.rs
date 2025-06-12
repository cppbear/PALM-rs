// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
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
    let class_ascii = ast::ClassAscii {
        span: ast::Span {},  // Assuming ast::Span has a default empty constructor
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer_instance.wtr.output, "[:^alnum:]");
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
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
    let class_ascii = ast::ClassAscii {
        span: ast::Span {},
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer_instance.wtr.output, "[:^alpha:]");
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
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
    let class_ascii = ast::ClassAscii {
        span: ast::Span {},
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer_instance.wtr.output, "[:^digit:]");
}

#[test]
fn test_fmt_class_ascii_space_not_negated() {
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
    let class_ascii = ast::ClassAscii {
        span: ast::Span {},
        kind: ast::ClassAsciiKind::Space,
        negated: false,
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    writer_instance.fmt_class_ascii(&class_ascii).unwrap();

    assert_eq!(writer_instance.wtr.output, "[:space:]");
}

