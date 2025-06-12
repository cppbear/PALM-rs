// Answer 0

#[test]
fn test_fmt_class_bracketed_pre_negated() {
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

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter { wtr: MockWriter::new() }
        }
        
        fn fmt_class_bracketed_pre(
            &mut self,
            ast: &ClassBracketed,
        ) -> fmt::Result {
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

    let mut formatter = Formatter::new();
    let ast = ClassBracketed { negated: true };

    let result = formatter.fmt_class_bracketed_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "[^");
}

