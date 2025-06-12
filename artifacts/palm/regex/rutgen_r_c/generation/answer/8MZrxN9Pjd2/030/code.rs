// Answer 0

#[test]
fn test_visit_pre_unicode_class_with_range() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: TestWriter { output: String::new() },
    };

    let unicode_range = ClassUnicodeRange::new('a', 'z');
    let unicode_class = ClassUnicode::new(vec![unicode_range.clone(), unicode_range.clone()]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Unicode(unicode_class)),
        info: HirInfo::default(),
    };

    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "[a-z]");
}

#[test]
fn test_visit_pre_byte_class_with_range() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: TestWriter { output: String::new() },
    };

    let byte_range = ClassBytesRange::new(65, 90); // A-Z
    let byte_class = ClassBytes::new(vec![byte_range.clone(), byte_range.clone()]);
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Bytes(byte_class)),
        info: HirInfo::default(),
    };

    let result = writer.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "(?-u:[A-Z])");
}

#[test]
#[should_panic]
fn test_visit_pre_unicode_literal_panic() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: TestWriter { output: String::new() },
    };

    let unicode_literal = Hir {
        kind: HirKind::Literal(hir::Literal::Unicode('g')),
        info: HirInfo::default(),
    };

    writer.visit_pre(&unicode_literal).expect("Should panic on write error");
}

#[test]
fn test_visit_pre_byte_literal() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: TestWriter { output: String::new() },
    };

    let byte_literal = Hir {
        kind: HirKind::Literal(hir::Literal::Byte(b'A')),
        info: HirInfo::default(),
    };

    let result = writer.visit_pre(&byte_literal);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "A");
}

