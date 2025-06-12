// Answer 0

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
        span: ast::Span::default(), // Assuming there's a default implementation
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    formatter.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^alpha:]");
}

#[test]
fn test_fmt_class_ascii_alpha_non_negated() {
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
        span: ast::Span::default(), // Assuming there's a default implementation
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };

    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    formatter.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:alpha:]");
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
        span: ast::Span::default(), // Assuming there's a default implementation
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    formatter.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^digit:]");
}

#[test]
fn test_fmt_class_ascii_space_non_negated() {
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
        span: ast::Span::default(), // Assuming there's a default implementation
        kind: ast::ClassAsciiKind::Space,
        negated: false,
    };

    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    formatter.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:space:]");
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
        span: ast::Span::default(), // Assuming there's a default implementation
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };

    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    formatter.fmt_class_ascii(&ast).unwrap();

    assert_eq!(writer.output, "[:^upper:]");
}

