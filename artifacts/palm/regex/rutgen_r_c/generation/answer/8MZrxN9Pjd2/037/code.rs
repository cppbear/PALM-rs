// Answer 0

#[test]
fn test_visit_pre_empty_hir() {
    struct TestWriter {
        content: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { content: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert!(writer.content.is_empty());
}

#[test]
fn test_visit_pre_bytes_class() {
    struct TestWriter {
        content: String,
        should_fail: bool,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            self.content.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { content: String::new(), should_fail: true };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let bytes_class = ClassBytes::new(vec![ClassBytesRange::new(0b00000001, 0b00000001)]);
    let hir = Hir { kind: HirKind::Class(hir::Class::Bytes(bytes_class)), info: HirInfo::default() };
    let result = visitor.visit_pre(&hir);
    assert!(result.is_err());
}

#[test]
fn test_visit_pre_unicode_class() {
    struct TestWriter {
        content: String,
        should_fail: bool,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            self.content.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { content: String::new(), should_fail: true };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let unicode_class = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'a')]);
    let hir = Hir { kind: HirKind::Class(hir::Class::Unicode(unicode_class)), info: HirInfo::default() };
    let result = visitor.visit_pre(&hir);
    assert!(result.is_err());
}

