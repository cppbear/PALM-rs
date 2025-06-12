// Answer 0

#[test]
fn visit_pre_test_empty() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    assert_eq!(visitor.visit_pre(&hir), Ok(()));
}

#[test]
fn visit_pre_test_literal_unicode() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir { kind: HirKind::Literal(hir::Literal::Unicode('a')), info: HirInfo::default() };
    assert_eq!(visitor.visit_pre(&hir), Ok(()));
    assert_eq!(writer.output, "a");
}

#[test]
fn visit_pre_test_literal_byte() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir { kind: HirKind::Literal(hir::Literal::Byte(b'a')), info: HirInfo::default() };
    assert_eq!(visitor.visit_pre(&hir), Ok(()));
    assert_eq!(writer.output, "a");
}

#[test]
fn visit_pre_test_anchor_start_line() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartLine), info: HirInfo::default() };
    assert_eq!(visitor.visit_pre(&hir), Ok(()));
    assert_eq!(writer.output, "(?m:^)");
}

#[test]
fn visit_pre_test_anchor_end_line() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndLine), info: HirInfo::default() };
    assert_eq!(visitor.visit_pre(&hir), Ok(()));
    assert_eq!(writer.output, "(?m:$)");
}

