// Answer 0

#[test]
fn test_visit_post_repetition() {
    use regex_syntax::ast::{Ast, Repetition};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter {
        wtr: TestWriter,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { wtr: TestWriter::new() }
        }

        fn fmt_repetition(&mut self, _: &Repetition) -> std::fmt::Result {
            self.wtr.write_str("repetition")
        }

        fn visit_post(&mut self, ast: &Ast) -> std::fmt::Result {
            match *ast {
                Ast::Repetition(ref x) => self.fmt_repetition(x),
                _ => Ok(()),
            }
        }
    }

    let mut formatter = TestFormatter::new();
    let repetition_ast = Ast::Repetition(Repetition::new(/* parameters as needed */));
    let result = formatter.visit_post(&repetition_ast);

    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "repetition");
}

