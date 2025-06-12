// Answer 0

#[test]
fn test_fmt_class_bracketed_pre_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
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

        fn fmt_class_bracketed_pre(&mut self, ast: &ClassBracketed) -> std::fmt::Result {
            if ast.negated {
                self.wtr.write_str("[^")
            } else {
                self.wtr.write_str("[")
            }
        }
    }

    struct ClassBracketed {
        negated: bool,
    }

    let mut formatter = MockFormatter::new();
    let ast = ClassBracketed { negated: true };
    formatter.fmt_class_bracketed_pre(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[^");
}

#[test]
fn test_fmt_class_bracketed_pre_not_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
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

        fn fmt_class_bracketed_pre(&mut self, ast: &ClassBracketed) -> std::fmt::Result {
            if ast.negated {
                self.wtr.write_str("[^")
            } else {
                self.wtr.write_str("[")
            }
        }
    }

    struct ClassBracketed {
        negated: bool,
    }

    let mut formatter = MockFormatter::new();
    let ast = ClassBracketed { negated: false };
    formatter.fmt_class_bracketed_pre(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "[");
}

