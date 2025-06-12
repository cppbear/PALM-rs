// Answer 0

fn test_visit_pre_start_line() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                wtr: MockWriter { output: String::new() },
            }
        }

        fn write_literal_char(&mut self, _: char) -> fmt::Result {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::StartLine));
    let result = visitor.visit_pre(&hir);

    assert_eq!(result, Ok(()));
    assert_eq!(visitor.wtr.output, "(?m:^)");
    Ok(())
}

fn test_visit_pre_end_line() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                wtr: MockWriter { output: String::new() },
            }
        }

        fn write_literal_char(&mut self, _: char) -> fmt::Result {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::EndLine));
    let result = visitor.visit_pre(&hir);

    assert_eq!(result, Ok(()));
    assert_eq!(visitor.wtr.output, "(?m:$)");
    Ok(())
}

fn test_visit_pre_start_text() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                wtr: MockWriter { output: String::new() },
            }
        }

        fn write_literal_char(&mut self, _: char) -> fmt::Result {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::StartText));
    let result = visitor.visit_pre(&hir);

    assert_eq!(result, Ok(()));
    assert_eq!(visitor.wtr.output, r"\A");
    Ok(())
}

fn test_visit_pre_end_text() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Err(std::fmt::Error)
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                wtr: MockWriter { output: String::new() },
            }
        }

        fn write_literal_char(&mut self, _: char) -> fmt::Result {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::EndText));
    let result = visitor.visit_pre(&hir);

    assert!(result.is_err());
    Ok(())
}

