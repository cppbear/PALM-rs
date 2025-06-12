// Answer 0

#[test]
fn test_fmt_assertion_word_boundary() {
    use std::fmt;
    use std::io::Write;

    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }
    }

    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    mod ast {
        pub enum AssertionKind {
            StartLine,
            EndLine,
            StartText,
            EndText,
            WordBoundary,
            NotWordBoundary,
        }

        pub struct Assertion {
            pub kind: AssertionKind,
        }
    }

    struct TestFormatter {
        wtr: Writer,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { wtr: Writer::new() }
        }

        fn fmt_assertion(&mut self, ast: &ast::Assertion) -> fmt::Result {
            use ast::AssertionKind::*;
            match ast.kind {
                StartLine => self.wtr.write_str("^"),
                EndLine => self.wtr.write_str("$"),
                StartText => self.wtr.write_str(r"\A"),
                EndText => self.wtr.write_str(r"\z"),
                WordBoundary => self.wtr.write_str(r"\b"),
                NotWordBoundary => self.wtr.write_str(r"\B"),
            }
        }
    }

    let mut formatter = TestFormatter::new();
    let assertion = ast::Assertion { kind: ast::AssertionKind::WordBoundary };
    let result = formatter.fmt_assertion(&assertion);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, r"\b");
}

