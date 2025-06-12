// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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
                wtr: TestWriter {
                    output: String::new(),
                },
            }
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate));

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(visitor.wtr.output, r"(?-u:\B)");
}

#[test]
fn test_visit_pre_anchor_start_line() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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
                wtr: TestWriter {
                    output: String::new(),
                },
            }
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::StartLine));

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(visitor.wtr.output, "(?m:^)");
}

#[test]
fn test_visit_pre_anchor_end_line() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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
                wtr: TestWriter {
                    output: String::new(),
                },
            }
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::EndLine));

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(visitor.wtr.output, "(?m:$)");
}

#[test]
fn test_visit_pre_word_boundary_ascii() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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
                wtr: TestWriter {
                    output: String::new(),
                },
            }
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::Ascii));

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(visitor.wtr.output, "(?-u:\\b)");
}

#[test]
fn test_visit_pre_word_boundary_ascii_negate() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
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
                wtr: TestWriter {
                    output: String::new(),
                },
            }
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::AsciiNegate));

    assert!(visitor.visit_pre(&hir).is_ok());
    assert_eq!(visitor.wtr.output, "(?-u:\\B)");
}

