// Answer 0

fn test_visit_post_one_or_more_greedy_false() {
    use regex_syntax::hir::{self, Hir, HirKind};
    use std::fmt::Write;

    struct TestWriter {
        buf: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { buf: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buf.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let mut visitor = Visitor { wtr: &mut writer };

    let repetition = hir::Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: false,
    };

    let hir = Hir::from(HirKind::Repetition(repetition));

    let result = visitor.visit_post(&hir);

    assert_eq!(result, Ok(()));
    assert_eq!(writer.buf, "+?");
}

