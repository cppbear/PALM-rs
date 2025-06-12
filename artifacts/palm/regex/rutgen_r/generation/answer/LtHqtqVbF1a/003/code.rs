// Answer 0

#[test]
fn test_fmt_repetition_range() {
    use regex_syntax::ast::{self, Repetition, RepetitionKind};
    use std::fmt::{self, Write};

    struct MockWriter {
        output: String,
        should_error: bool,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_error {
                Err(fmt::Error)
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    struct MockFormatter {
        wtr: MockWriter,
    }

    impl MockFormatter {
        fn new(should_error: bool) -> Self {
            Self {
                wtr: MockWriter {
                    output: String::new(),
                    should_error,
                },
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

        fn fmt_repetition_range(&mut self, _range: &ast::Range) -> fmt::Result {
            Ok(())
        }
    }

    let range = ast::Range::default(); // Assuming a default method exists
    let ast_repetition = Repetition {
        op: ast::RepetitionOp {
            kind: RepetitionKind::Range(Box::new(range)),
        },
        greedy: false,
    };

    let mut formatter = MockFormatter::new(true);
    
    let result = formatter.fmt_repetition(&ast_repetition);
    
    assert!(result.is_err());
}

