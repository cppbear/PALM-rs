// Answer 0

#[test]
fn test_fmt_class_unicode_negated_named_value_not_equal() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ast::ClassUnicode {
        span: Span::new(0, 0),
        negated: true,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::NotEqual,
            name: String::from("name"),
            value: String::from("value"),
        },
    };
    
    let result = writer_ref.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\P{name!=value}");
}

#[test]
fn test_fmt_class_unicode_not_negated_named_value_equal() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ast::ClassUnicode {
        span: Span::new(0, 0),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Equal,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let result = writer_ref.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\p{name=value}");
}

#[test]
fn test_fmt_class_unicode_not_negated_named_value_colon() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ast::ClassUnicode {
        span: Span::new(0, 0),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Colon,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let result = writer_ref.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\p{name:value}");
}

#[test]
fn test_fmt_class_unicode_not_negated_named() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };

    let ast = ast::ClassUnicode {
        span: Span::new(0, 0),
        negated: false,
        kind: ast::ClassUnicodeKind::Named(String::from("name")),
    };

    let result = writer_ref.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\p{name}");
}

