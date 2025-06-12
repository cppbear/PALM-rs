// Answer 0

#[test]
fn test_fmt_class_ascii_print_not_negated() {
    use regex_syntax::ast::{ClassAscii, ClassAsciiKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    
    let ast = ClassAscii {
        kind: ClassAsciiKind::Print,
        negated: false,
    };
    
    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:print:]");
}

#[test]
fn test_fmt_class_ascii_print_negated() {
    use regex_syntax::ast::{ClassAscii, ClassAsciiKind};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    
    let ast = ClassAscii {
        kind: ClassAsciiKind::Print,
        negated: true,
    };
    
    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:^print:]");
}

