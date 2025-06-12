// Answer 0

#[test]
fn test_visit_post_exactly_one_not_greedy() {
    use std::fmt::Write; // For using write_str

    struct MockWriter {
        string: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                string: String::new(),
            }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.string.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                wtr: MockWriter::new(),
            }
        }

        fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Repetition(ref x) => {
                    match x.kind {
                        hir::RepetitionKind::Range(ref range) => {
                            match *range {
                                hir::RepetitionRange::Exactly(m) => {
                                    write!(self.wtr, "{{{}}}", m)?;
                                },
                                _ => {},
                            }
                        },
                        _ => {},
                    }
                    if !x.greedy {
                        self.wtr.write_str("?")?;
                    }
                },
                _ => {},
            }
            Ok(())
        }
    }

    // Define the necessary context to create Hir and its children
    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn new(kind: HirKind) -> Self {
            Hir { kind }
        }

        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    enum HirKind {
        Repetition(Repetition),
    }

    struct Repetition {
        kind: hir::RepetitionKind,
        greedy: bool,
    }

    mod hir {
        pub enum RepetitionKind {
            Range(Box<RepetitionRange>),
        }

        pub enum RepetitionRange {
            Exactly(usize),
        }
    }

    // Test input setup
    let repetition = Repetition {
        kind: hir::RepetitionKind::Range(Box::new(hir::RepetitionRange::Exactly(1))),
        greedy: false,
    };
    let hir = Hir::new(HirKind::Repetition(repetition));

    // Execute the function under test
    let mut visitor = MockVisitor::new();
    let result = visitor.visit_post(&hir);

    // Assert that we get the expected result and output
    assert_eq!(result, Ok(()));
    assert_eq!(visitor.wtr.string, "{{1}}?");
}

