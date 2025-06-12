// Answer 0

#[test]
fn test_visit_post_empty() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind};

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };
    let hir = Hir::new_empty();

    let result = visitor.visit_post(&hir);
    
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "");
}

#[test]
fn test_visit_post_literal() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind};

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };
    let hir = Hir::new_literal("abc");

    let result = visitor.visit_post(&hir);
    
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "");
}

#[test]
fn test_visit_post_repetition_zero_or_one() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind, RepetitionKind};

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };
    let hir = Hir::new_repetition(RepetitionKind::ZeroOrOne, false);

    let result = visitor.visit_post(&hir);
    
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "?");
}

#[test]
fn test_visit_post_repetition_zero_or_more() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind, RepetitionKind};

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };
    let hir = Hir::new_repetition(RepetitionKind::ZeroOrMore, false);

    let result = visitor.visit_post(&hir);
    
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "*");
}

#[test]
fn test_visit_post_repetition_one_or_more() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    use regex_syntax::hir::{Hir, HirKind, RepetitionKind};

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };
    let hir = Hir::new_repetition(RepetitionKind::OneOrMore, false);

    let result = visitor.visit_post(&hir);
    
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "+");
}

