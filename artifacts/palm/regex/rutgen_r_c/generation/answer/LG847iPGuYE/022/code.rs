// Answer 0

#[test]
fn test_visit_post_repetition_zero_or_more() {
    use hir::{Hir, HirKind, Repetition, RepetitionKind};
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

    let mut test_writer = TestWriter { output: String::new() };

    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: true,
            hir: Box::new(Hir::empty()), // Nested Hir can be empty for this test
        }),
        info: Default::default(), // Assume default initialization of info
    };

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: test_writer,
    };

    let result = writer.visit_post(&repetition_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.wtr.output, "*");
}

#[test]
fn test_visit_post_repetition_zero_or_more_non_greedy() {
    use hir::{Hir, HirKind, Repetition, RepetitionKind};
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

    let mut test_writer = TestWriter { output: String::new() };

    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            kind: RepetitionKind::ZeroOrMore,
            greedy: false,
            hir: Box::new(Hir::empty()), // Nested Hir can be empty for this test
        }),
        info: Default::default(), // Assume default initialization of info
    };

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: test_writer,
    };

    let result = writer.visit_post(&repetition_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.wtr.output, "*?");
}

