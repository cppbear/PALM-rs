// Answer 0

fn test_visit_pre_empty() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn get_output(&self) -> &String {
            &self.output
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

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            // Call the method under test
            visit_pre(self, hir)
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::Empty);
    visitor.visit_pre(&hir).unwrap();
    assert_eq!(visitor.wtr.get_output(), "");
}

fn test_visit_pre_word_boundary_unicode() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn get_output(&self) -> &String {
            &self.output
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

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            // Call the method under test
            visit_pre(self, hir)
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::Unicode));
    visitor.visit_pre(&hir).unwrap();
    assert_eq!(visitor.wtr.get_output(), r"\b");
}

fn test_visit_pre_word_boundary_unicode_negate() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn get_output(&self) -> &String {
            &self.output
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

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            // Call the method under test
            visit_pre(self, hir)
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate));
    visitor.visit_pre(&hir).unwrap();
    assert_eq!(visitor.wtr.get_output(), r"\B");
}

fn test_visit_pre_word_boundary_ascii() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn get_output(&self) -> &String {
            &self.output
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

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            // Call the method under test
            visit_pre(self, hir)
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::Ascii));
    visitor.visit_pre(&hir).unwrap();
    assert_eq!(visitor.wtr.get_output(), r"(?-u:\b)");
}

fn test_visit_pre_word_boundary_ascii_negate() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn get_output(&self) -> &String {
            &self.output
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

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            // Call the method under test
            visit_pre(self, hir)
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::AsciiNegate));
    visitor.visit_pre(&hir).unwrap();
    assert_eq!(visitor.wtr.get_output(), r"(?-u:\B)");
}

