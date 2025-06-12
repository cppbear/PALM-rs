// Answer 0

#[test]
fn test_visit_post_zero_or_one_repetition() {
    use std::fmt;
    use regex_syntax::hir::{Hir, HirKind, Repetition, RepetitionKind, RepetitionRange};

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

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl TestVisitor<'_> {
        fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
            visit_post(self, hir)
        }
    }

    let mut writer = TestWriter { output: String::new(), should_fail: true };
    let mut visitor = TestVisitor { wtr: &mut writer };

    let repetition = Repetition {
        kind: RepetitionKind::ZeroOrOne,
        greedy: true,
    };
    let hir = Hir::from_kind(HirKind::Repetition(repetition));

    let result = visitor.visit_post(&hir);
    assert!(result.is_err());
}

