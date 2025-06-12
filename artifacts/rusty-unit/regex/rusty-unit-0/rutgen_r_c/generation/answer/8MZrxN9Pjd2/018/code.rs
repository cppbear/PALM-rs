// Answer 0

#[test]
fn test_visit_pre_start_line() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::StartLine),
        info: HirInfo::default(),
    };

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, "(?m:^)");
}

#[test]
fn test_visit_pre_end_line() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::EndLine),
        info: HirInfo::default(),
    };

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, "(?m:$)");
}

#[test]
fn test_visit_pre_start_text() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::StartText),
        info: HirInfo::default(),
    };

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, r"\A");
}

#[test]
fn test_visit_pre_end_text() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir {
        kind: HirKind::Anchor(hir::Anchor::EndText),
        info: HirInfo::default(),
    };

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, r"\z");
}

#[test]
fn test_visit_pre_empty() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo::default(),
    };

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(writer.output, "");
}

