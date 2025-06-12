// Answer 0

#[derive(Debug)]
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

struct MockFormatter<'a> {
    wtr: &'a mut MockWriter,
}

impl<'a> MockFormatter<'a> {
    fn fmt_assertion(&mut self, ast: &Assertion) -> std::fmt::Result {
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

mod ast {
    pub struct Assertion {
        pub kind: AssertionKind,
    }

    #[derive(Debug)]
    pub enum AssertionKind {
        StartLine,
        EndLine,
        StartText,
        EndText,
        WordBoundary,
        NotWordBoundary,
    }
}

#[test]
fn test_fmt_assertion_start_text() {
    let mut writer = MockWriter::new();
    let mut formatter = MockFormatter { wtr: &mut writer };
    
    let assertion = ast::Assertion { kind: ast::AssertionKind::StartText };
    
    let result = formatter.fmt_assertion(&assertion);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\A");
}

