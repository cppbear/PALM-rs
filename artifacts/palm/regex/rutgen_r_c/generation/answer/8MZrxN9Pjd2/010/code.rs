// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter(String::new());
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate), info: Default::default() };

    writer.visit_pre(&hir).unwrap();
    assert_eq!(writer.wtr.0, "(?-u:\\B)");
}

#[test]
fn test_visit_pre_word_boundary_ascii() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter(String::new());
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::Ascii), info: Default::default() };

    writer.visit_pre(&hir).unwrap();
    assert_eq!(writer.wtr.0, "(?-u:\\b)");
}

#[test]
fn test_visit_pre_word_boundary_ascii_negate() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter(String::new());
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };
    let hir = Hir { kind: HirKind::WordBoundary(hir::WordBoundary::AsciiNegate), info: Default::default() };

    writer.visit_pre(&hir).unwrap();
    assert_eq!(writer.wtr.0, "(?-u:\\B)");
}

#[test]
fn test_visit_pre_anchor_start_line() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter(String::new());
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartLine), info: Default::default() };

    writer.visit_pre(&hir).unwrap();
    assert_eq!(writer.wtr.0, "(?m:^)");
}

#[test]
fn test_visit_pre_anchor_end_line() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter(String::new());
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndLine), info: Default::default() };

    writer.visit_pre(&hir).unwrap();
    assert_eq!(writer.wtr.0, "(?m:$)");
}

#[test]
fn test_visit_pre_anchor_start_text() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter(String::new());
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::StartText), info: Default::default() };

    writer.visit_pre(&hir).unwrap();
    assert_eq!(writer.wtr.0, r"\A");
}

#[test]
fn test_visit_pre_anchor_end_text() {
    struct MockWriter(String);
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = MockWriter(String::new());
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: output };
    let hir = Hir { kind: HirKind::Anchor(hir::Anchor::EndText), info: Default::default() };

    writer.visit_pre(&hir).unwrap();
    assert_eq!(writer.wtr.0, r"\z");
}

