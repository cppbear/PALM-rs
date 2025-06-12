// Answer 0

fn test_visit_post_zero_or_one() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrOne,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };

    visitor.visit_post(&hir).unwrap();
    assert_eq!(visitor.wtr.output, "?");
}

fn test_visit_post_one_or_more() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::OneOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };

    visitor.visit_post(&hir).unwrap();
    assert_eq!(visitor.wtr.output, "+");
}

fn test_visit_post_zero_or_more() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };

    visitor.visit_post(&hir).unwrap();
    assert_eq!(visitor.wtr.output, "*");
}

fn test_visit_post_zero_or_one_greedy() {
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
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrOne,
            greedy: true,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };

    visitor.visit_post(&hir).unwrap();
    assert_eq!(visitor.wtr.output, "");
}

fn test_visit_post_zero_or_more_err() {
    struct TestWriter {
        output: String,
        should_fail: bool,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                return Err(fmt::Error);
            }
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new(), should_fail: true };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()),
        }),
        info: Default::default(),
    };

    let result = visitor.visit_post(&hir);
    assert!(result.is_err());
}

