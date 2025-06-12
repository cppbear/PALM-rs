// Answer 0

fn test_fmt_class_unicode_one_letter() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let ast = ast::ClassUnicode {
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };

    let result = writer.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\pa");
}

fn test_fmt_class_unicode_named_value_equal() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let ast = ast::ClassUnicode {
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Equal,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let result = writer.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\p{name=value}");
}

fn test_fmt_class_unicode_named_value_colon() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let ast = ast::ClassUnicode {
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Colon,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let result = writer.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\p{name:value}");
}

fn test_fmt_class_unicode_named_value_not_equal() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let ast = ast::ClassUnicode {
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::NotEqual,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let result = writer.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\p{name!=value}");
}

