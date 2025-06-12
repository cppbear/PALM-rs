// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode() {
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
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::Unicode) };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\b");
}

#[test]
fn test_visit_pre_class_unicode() {
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
    let unicode_range = ClassUnicodeRange::new('a', 'z');
    let class_unicode = ClassUnicode::new(vec![unicode_range.clone()]);
    let hir = Hir { kind: HirKind::Class(hir::Class::Unicode(class_unicode)) };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(writer.output, "[a-z]");
}

#[test]
fn test_visit_pre_class_bytes() {
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
    let bytes_range = ClassBytesRange::new(1, 3);
    let class_bytes = ClassBytes::new(vec![bytes_range.clone()]);
    let hir = Hir { kind: HirKind::Class(hir::Class::Bytes(class_bytes)) };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(writer.output, "(?-u:[\x01-\x03])");
}

#[test]
fn test_visit_pre_word_boundary_ascii() {
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
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::Ascii) };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(writer.output, r"(?-u:\b)");
}

