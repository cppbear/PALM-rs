// Answer 0

#[test]
fn test_visit_post_repetition_exactly() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(3)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    
    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };

    assert!(writer_ref.visit_post(&hir).is_ok());
    assert_eq!(writer.output, "{{3}}");
}

#[test]
fn test_visit_post_repetition_at_least() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(2)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    
    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };

    assert!(writer_ref.visit_post(&hir).is_ok());
    assert_eq!(writer.output, "{{2,}}");
}

#[test]
fn test_visit_post_repetition_bounded() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)),
        greedy: true,
        hir: Box::new(Hir::empty()),
    };

    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };

    assert!(writer_ref.visit_post(&hir).is_ok());
    assert_eq!(writer.output, "{{2,5}}");
}

#[test]
fn test_visit_post_repetition_non_greedy() {
    use std::fmt::Write;

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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(Hir::empty()),
    };

    let hir = Hir { kind: HirKind::Repetition(repetition), info: HirInfo::default() };

    assert!(writer_ref.visit_post(&hir).is_ok());
    assert_eq!(writer.output, "*?");
}

