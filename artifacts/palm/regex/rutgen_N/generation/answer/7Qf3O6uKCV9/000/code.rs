// Answer 0

#[test]
fn test_fmt_assertion_start_line() {
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
    
    let mut writer = MockWriter::new();
    let ast = ast::Assertion { kind: ast::AssertionKind::StartLine };
    writer.fmt_assertion(&ast).unwrap();
    assert_eq!(writer.output, "^");
}

#[test]
fn test_fmt_assertion_end_line() {
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
    
    let mut writer = MockWriter::new();
    let ast = ast::Assertion { kind: ast::AssertionKind::EndLine };
    writer.fmt_assertion(&ast).unwrap();
    assert_eq!(writer.output, "$");
}

#[test]
fn test_fmt_assertion_start_text() {
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
    
    let mut writer = MockWriter::new();
    let ast = ast::Assertion { kind: ast::AssertionKind::StartText };
    writer.fmt_assertion(&ast).unwrap();
    assert_eq!(writer.output, r"\A");
}

#[test]
fn test_fmt_assertion_end_text() {
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
    
    let mut writer = MockWriter::new();
    let ast = ast::Assertion { kind: ast::AssertionKind::EndText };
    writer.fmt_assertion(&ast).unwrap();
    assert_eq!(writer.output, r"\z");
}

#[test]
fn test_fmt_assertion_word_boundary() {
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
    
    let mut writer = MockWriter::new();
    let ast = ast::Assertion { kind: ast::AssertionKind::WordBoundary };
    writer.fmt_assertion(&ast).unwrap();
    assert_eq!(writer.output, r"\b");
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
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
    
    let mut writer = MockWriter::new();
    let ast = ast::Assertion { kind: ast::AssertionKind::NotWordBoundary };
    writer.fmt_assertion(&ast).unwrap();
    assert_eq!(writer.output, r"\B");
}

