// Answer 0

fn test_visit_post_repetition_range_at_least() {
    use regex_syntax::hir::{self, Hir, HirKind, Repetition, RepetitionKind, RepetitionRange};
    use std::fmt;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor {
                wtr: TestWriter { output: String::new() },
            }
        }

        fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Repetition(ref x) => {
                    match x.kind {
                        RepetitionKind::Range(ref range) => {
                            match *range {
                                RepetitionRange::AtLeast(m) => {
                                    if m == 0 {
                                        return Err(fmt::Error);
                                    }
                                    write!(self.wtr, "{{{},}}", m)?;
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

    let mut visitor = TestVisitor::new();
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(1)),
        greedy: false,
    };
    let hir = Hir::from_kind(HirKind::Repetition(repetition));
    
    let result = visitor.visit_post(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "{{1,}}?");
}

fn test_visit_post_repetition_range_none() {
    use regex_syntax::hir::{self, Hir, HirKind, Repetition, RepetitionKind, RepetitionRange};
    use std::fmt;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor {
                wtr: TestWriter { output: String::new() },
            }
        }

        fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Repetition(ref x) => {
                    match x.kind {
                        RepetitionKind::Range(ref range) => {
                            match *range {
                                RepetitionRange::AtLeast(m) => {
                                    if m < 0 {
                                        return Err(fmt::Error);
                                    }
                                    write!(self.wtr, "{{{},}}", m)?;
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

    let mut visitor = TestVisitor::new();
    let repetition = Repetition {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(-1)),
        greedy: false,
    };
    let hir = Hir::from_kind(HirKind::Repetition(repetition));
    
    let result = visitor.visit_post(&hir);
    assert!(result.is_err());
}

