// Answer 0

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
    }

    struct MockHir {
        kind: HirKind,
    }

    let mut writer = MockWriter::new();
    let hir = MockHir { kind: HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate) };

    let result = visit_pre(&mut writer, &hir);

    assert!(result.is_ok());
    assert_eq!(writer.output, r"\B");
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
    }

    struct MockHir {
        kind: HirKind,
    }

    let mut writer = MockWriter::new();
    let hir = MockHir { kind: HirKind::WordBoundary(hir::WordBoundary::Ascii) };

    let result = visit_pre(&mut writer, &hir);

    assert!(result.is_ok());
    assert_eq!(writer.output, r"(?-u:\b)");
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
    }

    struct MockHir {
        kind: HirKind,
    }

    let mut writer = MockWriter::new();
    let hir = MockHir { kind: HirKind::WordBoundary(hir::WordBoundary::AsciiNegate) };

    let result = visit_pre(&mut writer, &hir);

    assert!(result.is_ok());
    assert_eq!(writer.output, r"(?-u:\B)");
}

fn test_visit_pre_literal_byte() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
    }

    struct MockHir {
        kind: HirKind,
    }

    let mut writer = MockWriter::new();
    let hir = MockHir { kind: HirKind::Literal(hir::Literal::Byte(b'a')) };

    let result = visit_pre(&mut writer, &hir);

    assert!(result.is_ok());
    assert_eq!(writer.output, "a");
}

fn test_visit_pre_literal_unicode() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    struct MockHir {
        kind: HirKind,
    }

    let mut writer = MockWriter::new();
    let hir = MockHir { kind: HirKind::Literal(hir::Literal::Unicode('a')) };

    let result = visit_pre(&mut writer, &hir);

    assert!(result.is_ok());
    assert_eq!(writer.output, "a");
}

