// Answer 0

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter<'a> {
        wtr: TestWriter,
    }

    impl<'a> TestFormatter<'a> {
        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ZeroOrOne if ast.greedy => self.wtr.write_str("?"),
                ZeroOrOne => self.wtr.write_str("??"),
                ZeroOrMore if ast.greedy => self.wtr.write_str("*"),
                ZeroOrMore => self.wtr.write_str("*?"),
                OneOrMore if ast.greedy => self.wtr.write_str("+"),
                OneOrMore => self.wtr.write_str("+?"),
                Range(ref x) => {
                    self.fmt_repetition_range(x)?;
                    if !ast.greedy {
                        self.wtr.write_str("?")?;
                    }
                    Ok(())
                }
            }
        }

        fn fmt_repetition_range(&self, _range: &ast::Range) -> fmt::Result {
            // Assuming the range formatting is trivial for this test.
            Ok(())
        }
    }

    mod ast {
        pub struct Repetition {
            pub op: RepetitionOp,
            pub greedy: bool,
        }

        pub struct RepetitionOp {
            pub kind: RepetitionKind,
        }

        pub enum RepetitionKind {
            ZeroOrOne,
            ZeroOrMore,
            OneOrMore,
            Range(Box<Range>),
        }

        pub struct Range; // A placeholder for the Range struct.
    }

    let mut output_writer = TestWriter::new();
    let mut formatter = TestFormatter { wtr: output_writer };

    let ast = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: false,
    };

    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "+?");
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter<'a> {
        wtr: TestWriter,
    }

    impl<'a> TestFormatter<'a> {
        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ZeroOrOne if ast.greedy => self.wtr.write_str("?"),
                ZeroOrOne => self.wtr.write_str("??"),
                ZeroOrMore if ast.greedy => self.wtr.write_str("*"),
                ZeroOrMore => self.wtr.write_str("*?"),
                OneOrMore if ast.greedy => self.wtr.write_str("+"),
                OneOrMore => self.wtr.write_str("+?"),
                Range(ref x) => {
                    self.fmt_repetition_range(x)?;
                    if !ast.greedy {
                        self.wtr.write_str("?")?;
                    }
                    Ok(())
                }
            }
        }

        fn fmt_repetition_range(&self, _range: &ast::Range) -> fmt::Result {
            // Assuming the range formatting is trivial for this test.
            Ok(())
        }
    }

    mod ast {
        pub struct Repetition {
            pub op: RepetitionOp,
            pub greedy: bool,
        }

        pub struct RepetitionOp {
            pub kind: RepetitionKind,
        }

        pub enum RepetitionKind {
            ZeroOrOne,
            ZeroOrMore,
            OneOrMore,
            Range(Box<Range>),
        }

        pub struct Range; // A placeholder for the Range struct.
    }

    let mut output_writer = TestWriter::new();
    let mut formatter = TestFormatter { wtr: output_writer };

    let ast = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
    };

    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "+");
}

