// Answer 0

fn test_fmt_assertion_end_text() {
    use std::fmt;
    use std::io::Cursor;

    struct TestWriter {
        wtr: Cursor<Vec<u8>>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                wtr: Cursor::new(Vec::new()),
            }
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

    mod ast {
        #[derive(Debug)]
        pub enum AssertionKind {
            StartLine,
            EndLine,
            StartText,
            EndText,
            WordBoundary,
            NotWordBoundary,
        }

        #[derive(Debug)]
        pub struct Assertion {
            pub kind: AssertionKind,
        }
    }

    let mut writer = TestWriter::new();
    let assertion = ast::Assertion { kind: ast::AssertionKind::EndText };
    
    let result = writer.fmt_assertion(&assertion);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(writer.wtr.into_inner()).unwrap(), r"\z");
}

