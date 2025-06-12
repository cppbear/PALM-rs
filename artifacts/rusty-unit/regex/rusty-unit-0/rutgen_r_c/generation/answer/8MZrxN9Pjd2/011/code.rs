// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
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
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate), info: HirInfo::default() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?-u:\B)");
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
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::Ascii), info: HirInfo::default() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?-u:\b)");
}

#[test]
fn test_visit_pre_word_boundary_ascii_negate() {
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
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::AsciiNegate), info: HirInfo::default() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?-u:\B)");
}

#[test]
fn test_visit_pre_anchor_start_line() {
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
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartLine), info: HirInfo::default() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?m:^)");
}

#[test]
fn test_visit_pre_anchor_end_line() {
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
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndLine), info: HirInfo::default() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?m:$)");
}

#[test]
fn test_visit_pre_anchor_start_text() {
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
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartText), info: HirInfo::default() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"\A");
}

#[test]
fn test_visit_pre_anchor_end_text() {
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
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndText), info: HirInfo::default() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&hir);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"\z");
}

