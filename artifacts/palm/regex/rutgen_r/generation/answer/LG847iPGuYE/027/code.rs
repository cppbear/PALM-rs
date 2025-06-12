// Answer 0

#[test]
fn test_visit_post_zero_or_one_greedy_false() {
    use regex_syntax::hir::{Hir, HirKind, Repetition, RepetitionKind, RepetitionRange};
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let kind = HirKind::Repetition(Repetition {
        kind: RepetitionKind::ZeroOrOne,
        greedy: false,
    });
    let hir = Hir::new(kind);

    writer.visit_post(&hir).unwrap();

    assert_eq!(writer.output, "?");
}

#[test]
#[should_panic]
fn test_visit_post_zero_or_one_greedy_false_write_err() {
    use regex_syntax::hir::{Hir, HirKind, Repetition, RepetitionKind};
    
    struct FaultyWriter;

    impl std::fmt::Write for FaultyWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Err(std::fmt::Error)
        }
    }

    let mut writer = FaultyWriter;
    let kind = HirKind::Repetition(Repetition {
        kind: RepetitionKind::ZeroOrOne,
        greedy: false,
    });
    let hir = Hir::new(kind);

    let _ = writer.visit_post(&hir); // This should panic due to write error.
}

#[test]
fn test_visit_post_zero_or_more_greedy_false() {
    use regex_syntax::hir::{Hir, HirKind, Repetition, RepetitionKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let kind = HirKind::Repetition(Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: false,
    });
    let hir = Hir::new(kind);

    writer.visit_post(&hir).unwrap();

    assert_eq!(writer.output, "*?");
}

#[test]
fn test_visit_post_one_or_more_greedy_false() {
    use regex_syntax::hir::{Hir, HirKind, Repetition, RepetitionKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let kind = HirKind::Repetition(Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: false,
    });
    let hir = Hir::new(kind);

    writer.visit_post(&hir).unwrap();

    assert_eq!(writer.output, "+?");
}

