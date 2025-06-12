// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
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

    let ast = ast::ClassAscii {
        span: ast::Span::default(), // Assuming Span has a default implementation
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^alnum:]");
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
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

    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^alpha:]");
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

    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^digit:]");
}

#[test]
fn test_fmt_class_ascii_space_negated() {
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

    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Space,
        negated: true,
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^space:]");
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
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

    let ast = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^upper:]");
}

