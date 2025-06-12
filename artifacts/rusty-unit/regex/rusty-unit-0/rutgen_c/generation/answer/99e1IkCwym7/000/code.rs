// Answer 0

#[test]
fn test_fmt_class_unicode_positive_one_letter() {
    struct WriterMock {
        output: String,
    }

    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };

    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_unicode(&ast).unwrap();

    assert_eq!(writer.output, r"\pa");
}

#[test]
fn test_fmt_class_unicode_negative_one_letter() {
    struct WriterMock {
        output: String,
    }

    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };

    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_unicode(&ast).unwrap();

    assert_eq!(writer.output, r"\Pa");
}

#[test]
fn test_fmt_class_unicode_named() {
    struct WriterMock {
        output: String,
    }

    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };

    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::Named("scx".to_string()),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_unicode(&ast).unwrap();

    assert_eq!(writer.output, r"\pscx");
}

#[test]
fn test_fmt_class_unicode_named_value_equal() {
    struct WriterMock {
        output: String,
    }

    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };

    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue { op: ast::ClassUnicodeOpKind::Equal, name: "scx".to_string(), value: "Latin".to_string() },
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_unicode(&ast).unwrap();

    assert_eq!(writer.output, r"\pscx=Latin");
}

#[test]
fn test_fmt_class_unicode_named_value_not_equal() {
    struct WriterMock {
        output: String,
    }

    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };

    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue { op: ast::ClassUnicodeOpKind::NotEqual, name: "scx".to_string(), value: "Cyrillic".to_string() },
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_class_unicode(&ast).unwrap();

    assert_eq!(writer.output, r"\pscx!=Cyrillic");
}

