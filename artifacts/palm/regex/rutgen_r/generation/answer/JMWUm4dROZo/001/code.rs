// Answer 0

#[test]
fn test_fmt_set_flags_write_str_err() {
    struct MockWriter {
        should_error: bool,
    }

    impl MockWriter {
        fn new(should_error: bool) -> Self {
            MockWriter { should_error }
        }

        fn write_str(&mut self, _s: &str) -> Result<(), std::io::Error> {
            if self.should_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
            } else {
                Ok(())
            }
        }
    }

    struct TestFormatter {
        wtr: MockWriter,
    }

    impl TestFormatter {
        fn new(wtr: MockWriter) -> Self {
            TestFormatter { wtr }
        }

        fn fmt_set_flags(&mut self, ast: &ast::SetFlags) -> std::fmt::Result {
            self.wtr.write_str("(?")?;
            self.fmt_flags(&ast.flags)?;
            self.wtr.write_str(")")?;
            Ok(())
        }

        fn fmt_flags(&self, _flags: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    struct MockAst {
        flags: String,
    }

    let ast = MockAst {
        flags: "i".to_string(), // Example flags
    };

    let mut formatter = TestFormatter::new(MockWriter::new(true)); // Simulate error
    let result = formatter.fmt_set_flags(&ast);

    assert!(result.is_err());
}

#[test]
fn test_fmt_set_flags_success() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> Result<(), std::io::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter {
        wtr: MockWriter,
    }

    impl TestFormatter {
        fn new(wtr: MockWriter) -> Self {
            TestFormatter { wtr }
        }

        fn fmt_set_flags(&mut self, ast: &ast::SetFlags) -> std::fmt::Result {
            self.wtr.write_str("(?")?;
            self.fmt_flags(&ast.flags)?;
            self.wtr.write_str(")")?;
            Ok(())
        }

        fn fmt_flags(&self, flags: &str) -> std::fmt::Result {
            self.wtr.write_str(flags)
        }
    }

    struct MockAst {
        flags: String,
    }

    let ast = MockAst {
        flags: "i".to_string(), // Example flags
    };

    let mut formatter = TestFormatter::new(MockWriter::new());
    let result = formatter.fmt_set_flags(&ast);
    
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "(?i)");
}

