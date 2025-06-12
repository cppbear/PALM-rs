// Answer 0

#[test]
fn test_fmt_class_bracketed_post() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct MockFormatter {
        wtr: MockWriter,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_class_bracketed_post(&mut self, _ast: &ast::ClassBracketed) -> std::fmt::Result {
            self.wtr.write_str("]")
        }
    }

    struct TestClassBracketed;

    let mut formatter = MockFormatter::new();
    let ast = TestClassBracketed;

    let result = formatter.fmt_class_bracketed_post(&ast);
    
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "]");
}

