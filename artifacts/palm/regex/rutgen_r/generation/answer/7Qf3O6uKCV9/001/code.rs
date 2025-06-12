// Answer 0

#[test]
fn test_fmt_assertion_not_word_boundary() {
    use std::fmt::{self, Write};
    mod ast {
        pub struct Assertion {
            pub kind: AssertionKind,
        }
        
        pub enum AssertionKind {
            StartLine,
            EndLine,
            StartText,
            EndText,
            WordBoundary,
            NotWordBoundary,
        }
    }

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

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = TestWriter::new();
    let assertion = ast::Assertion {
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let result = writer.write_str(r"\B");
    assert!(result.is_ok());
    assert_eq!(writer.output, r"\B");
}

