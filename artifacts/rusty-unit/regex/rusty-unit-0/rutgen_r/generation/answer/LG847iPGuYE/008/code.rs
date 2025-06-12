// Answer 0

#[test]
fn test_visit_post_with_bounded_repetition() {
    use regex_syntax::hir::{self, Hir, HirKind, RepetitionKind, RepetitionRange};
    use std::fmt::{self, Write};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor<'a> {
        wtr: &'a mut TestWriter,
    }

    impl<'a> TestVisitor<'a> {
        fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Repetition(ref x) => {
                    match x.kind {
                        RepetitionKind::Range(ref r) => {
                            match *r {
                                RepetitionRange::Bounded(m, n) => {
                                    write!(self.wtr, "{{{},{}}}", m, n)?;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    if !x.greedy {
                        self.wtr.write_str("?")?;
                    }
                }
                _ => {}
            }
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let mut visitor = TestVisitor { wtr: &mut writer };

    let repetition = hir::Repetition {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(2, 5)),
        greedy: false,
    };

    let hir = Hir::from_kind(HirKind::Repetition(repetition));

    let result = visitor.visit_post(&hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "{{2,5}}?");
}

