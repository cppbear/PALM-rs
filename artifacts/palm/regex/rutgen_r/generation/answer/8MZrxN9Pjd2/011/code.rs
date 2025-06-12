// Answer 0

#[test]
fn test_visit_pre_word_boundary_unicode_negate() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate));
    
    let result = visit_pre(&mut writer, &hir);

    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?-u:\B)");
}

#[test]
fn test_visit_pre_anchor_start_line() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::StartLine));
    
    let result = visit_pre(&mut writer, &hir);

    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "(?m:^)"); 
}

#[test]
fn test_visit_pre_anchor_end_line() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let hir = Hir::new(HirKind::Anchor(hir::Anchor::EndLine));
    
    let result = visit_pre(&mut writer, &hir);

    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "(?m:$)");
}

#[test]
fn test_visit_pre_word_boundary_asciinegate() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let hir = Hir::new(HirKind::WordBoundary(hir::WordBoundary::AsciiNegate));

    let result = visit_pre(&mut writer, &hir);

    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, r"(?-u:\B)");
}

