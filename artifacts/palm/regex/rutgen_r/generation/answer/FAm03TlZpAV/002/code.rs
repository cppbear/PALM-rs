// Answer 0

#[test]
fn test_fmt_class_bracketed_pre_non_negated() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    
    struct ClassBracketed {
        negated: bool,
    }

    let ast = ClassBracketed { negated: false };
    
    let result = writer.write_str(if ast.negated { "[^" } else { "[" });
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "[");
}

