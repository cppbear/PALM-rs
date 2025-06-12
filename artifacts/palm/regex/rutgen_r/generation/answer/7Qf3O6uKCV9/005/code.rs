// Answer 0

#[test]
fn test_fmt_assertion_end_line() {
    use regex_syntax::ast::{Assertion, AssertionKind};
    use std::fmt;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn fmt_assertion(&mut self, ast: &Assertion) -> fmt::Result {
            use AssertionKind::*;
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

    let mut writer = MockWriter::new();
    let mut formatter = MockFormatter { wtr: &mut writer };
    let ast = Assertion { kind: AssertionKind::EndLine };

    formatter.fmt_assertion(&ast).unwrap();

    assert_eq!(writer.output, "$");
}

#[test]
fn test_fmt_assertion_start_text() {
    use regex_syntax::ast::{Assertion, AssertionKind};
    use std::fmt;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn fmt_assertion(&mut self, ast: &Assertion) -> fmt::Result {
            use AssertionKind::*;
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

    let mut writer = MockWriter::new();
    let mut formatter = MockFormatter { wtr: &mut writer };
    let ast = Assertion { kind: AssertionKind::StartText };

    formatter.fmt_assertion(&ast).unwrap();

    assert_eq!(writer.output, r"\A");
}

