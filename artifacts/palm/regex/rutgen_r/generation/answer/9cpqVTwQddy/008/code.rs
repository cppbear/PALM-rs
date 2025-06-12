// Answer 0

#[test]
fn test_fmt_class_ascii_space_true() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Space,
        negated: false,
    };
    
    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:space:]");
}

#[test]
fn test_fmt_class_ascii_space_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    
    let ast = ast::ClassAscii {
        kind: ast::ClassAsciiKind::Space,
        negated: true,
    };
    
    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:^space:]");
}

