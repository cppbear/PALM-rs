// Answer 0

fn test_fmt_repetition_one_or_more_greedy() {
    use regex_syntax::ast::{self, Repetition, RepetitionKind};
    use std::fmt::{self, Write};

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

    struct Formatter {
        wtr: TestWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: TestWriter::new(),
            }
        }

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
    }

    let mut formatter = Formatter::new();
    let repetition = Repetition {
        op: ast::RepetitionOp {
            kind: RepetitionKind::OneOrMore,
        },
        greedy: true,
    };

    let result = formatter.fmt_repetition(&repetition);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "+");
}

fn test_fmt_repetition_one_or_more_non_greedy() {
    use regex_syntax::ast::{self, Repetition, RepetitionKind};
    use std::fmt::{self, Write};

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

    struct Formatter {
        wtr: TestWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: TestWriter::new(),
            }
        }

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
    }

    let mut formatter = Formatter::new();
    let repetition = Repetition {
        op: ast::RepetitionOp {
            kind: RepetitionKind::OneOrMore,
        },
        greedy: false,
    };

    let result = formatter.fmt_repetition(&repetition);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "+?");
}

