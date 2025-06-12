// Answer 0

#[test]
fn test_fmt_class_unicode_not_negated_one_letter() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = MockWriter(String::new());
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    };

    let result = writer_instance.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.0, r"\pa");
}

#[test]
fn test_fmt_class_unicode_not_negated_named_value_equal() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = MockWriter(String::new());
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::Equal,
            name: String::from("script"),
            value: String::from("Latin"),
        },
    };

    let result = writer_instance.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.0, r"\p{script=Latin}");
}

#[test]
fn test_fmt_class_unicode_not_negated_named_value_colon() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = MockWriter(String::new());
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::Colon,
            name: String::from("script"),
            value: String::from("Cyrillic"),
        },
    };

    let result = writer_instance.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.0, r"\p{script:Cyrillic}");
}

#[test]
fn test_fmt_class_unicode_not_negated_named_value_not_equal() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = MockWriter(String::new());
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            op: ClassUnicodeOpKind::NotEqual,
            name: String::from("script"),
            value: String::from("Arabic"),
        },
    };

    let result = writer_instance.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.0, r"\p{script!=Arabic}");
}

#[test]
fn test_fmt_class_unicode_negated_one_letter() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = MockWriter(String::new());
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ClassUnicodeKind::OneLetter('b'),
    };

    let result = writer_instance.fmt_class_unicode(&ast);
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.0, r"\Pb");
}

