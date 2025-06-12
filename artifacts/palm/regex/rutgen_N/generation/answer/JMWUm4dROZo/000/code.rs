// Answer 0

#[test]
fn test_fmt_set_flags() {
    use std::fmt;
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestAst {
        flags: String,
    }

    struct TestFormatter {
        wtr: MockWriter,
    }

    impl TestFormatter {
        fn fmt_set_flags(&mut self, ast: &TestAst) -> fmt::Result {
            self.wtr.write_str("(?")?;
            self.fmt_flags(&ast.flags)?;
            self.wtr.write_str(")")?;
            Ok(())
        }

        fn fmt_flags(&mut self, flags: &str) -> fmt::Result {
            self.wtr.write_str(flags)
        }
    }

    let mut formatter = TestFormatter {
        wtr: MockWriter::new(),
    };
    let ast = TestAst {
        flags: String::from("i"),
    };

    formatter.fmt_set_flags(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "(?i)");
}

#[test]
fn test_fmt_set_flags_empty() {
    use std::fmt;
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestAst {
        flags: String,
    }

    struct TestFormatter {
        wtr: MockWriter,
    }

    impl TestFormatter {
        fn fmt_set_flags(&mut self, ast: &TestAst) -> fmt::Result {
            self.wtr.write_str("(?")?;
            self.fmt_flags(&ast.flags)?;
            self.wtr.write_str(")")?;
            Ok(())
        }

        fn fmt_flags(&mut self, flags: &str) -> fmt::Result {
            self.wtr.write_str(flags)
        }
    }

    let mut formatter = TestFormatter {
        wtr: MockWriter::new(),
    };
    let ast = TestAst {
        flags: String::new(),
    };

    formatter.fmt_set_flags(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "(?)");
}

