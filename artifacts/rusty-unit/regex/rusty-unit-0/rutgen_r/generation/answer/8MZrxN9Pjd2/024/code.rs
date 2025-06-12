// Answer 0

fn test_visit_pre_empty() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        Anchor(Anchor),
    }

    enum Anchor {
        StartLine,
        EndLine,
        StartText,
        EndText,
    }

    struct Visitor {
        wtr: MockWriter,
    }

    let mut visitor = Visitor {
        wtr: MockWriter { output: String::new() },
    };
    
    let hir = Hir { kind: HirKind::Empty };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "");

    result
}

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

    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        Anchor(Anchor),
    }

    enum Anchor {
        StartLine,
        EndLine,
        StartText,
        EndText,
    }

    struct Visitor {
        wtr: MockWriter,
    }

    let mut visitor = Visitor {
        wtr: MockWriter { output: String::new() },
    };
    
    let hir = Hir { kind: HirKind::Anchor(Anchor::StartLine) };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "(?m:^)");

    result
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

    struct Hir {
        kind: HirKind,
    }

    enum HirKind {
        Empty,
        Anchor(Anchor),
    }

    enum Anchor {
        StartLine,
        EndLine,
        StartText,
        EndText,
    }

    struct Visitor {
        wtr: MockWriter,
    }

    let mut visitor = Visitor {
        wtr: MockWriter { output: String::new() },
    };
    
    let hir = Hir { kind: HirKind::Anchor(Anchor::EndLine) };

    let result = visitor.visit_pre(&hir);
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "(?m:$)");

    result
}

