// Answer 0

#[test]
fn test_visit_post_repetition_one_or_more_non_greedy() {
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

    // Create a repetition of kind OneOrMore with greedy set to false
    let repetition_hir = hir::Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: false,
        hir: Box::new(hir::Hir::empty()),
    };

    let hir_instance = hir::Hir {
        kind: hir::HirKind::Repetition(repetition_hir),
        info: Default::default(),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.visit_post(&hir_instance);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "+"); // Check for the expected output
}

#[test]
fn test_visit_post_repetition_one_or_more_greedy() {
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

    // Create a repetition of kind OneOrMore with greedy set to true
    let repetition_hir = hir::Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(hir::Hir::empty()),
    };

    let hir_instance = hir::Hir {
        kind: hir::HirKind::Repetition(repetition_hir),
        info: Default::default(),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.visit_post(&hir_instance);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "+"); // Check for the expected output
}

#[test]
fn test_visit_post_repetition_zero_or_more_non_greedy() {
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

    // Create a repetition of kind ZeroOrMore with greedy set to false
    let repetition_hir = hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(hir::Hir::empty()),
    };

    let hir_instance = hir::Hir {
        kind: hir::HirKind::Repetition(repetition_hir),
        info: Default::default(),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.visit_post(&hir_instance);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "*?"); // Check for output of both zero-or-more and not-greedy
}

#[test]
fn test_visit_post_repetition_at_least() {
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

    // Create a repetition of kind AtLeast with greedy set to false
    let repetition_hir = hir::Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::AtLeast(5))),
        greedy: false,
        hir: Box::new(hir::Hir::empty()),
    };

    let hir_instance = hir::Hir {
        kind: hir::HirKind::Repetition(repetition_hir),
        info: Default::default(),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.visit_post(&hir_instance);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "{5,}?"); // Check for the expected output
}

#[test]
fn test_visit_post_repetition_bounded() {
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

    // Create a repetition of kind Bounded with greedy set to false
    let repetition_hir = hir::Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Bounded(3, 7))),
        greedy: false,
        hir: Box::new(hir::Hir::empty()),
    };

    let hir_instance = hir::Hir {
        kind: hir::HirKind::Repetition(repetition_hir),
        info: Default::default(),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = writer_instance.visit_post(&hir_instance);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "{3,7}?"); // Check for the expected output
}

