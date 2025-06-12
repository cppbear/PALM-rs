// Answer 0

#[test]
fn test_fmt_repetition_range_greedy() {
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

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }

        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> std::fmt::Result {
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

        fn fmt_repetition_range(&mut self, _x: &ast::RepetitionRange) -> std::fmt::Result {
            self.wtr.write_str("{1,5}") // Example implementation
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
            Range(RepetitionRange),
        }

        pub struct RepetitionRange;
    }

    let mut formatter = Formatter::new();
    let ast = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange),
        },
        greedy: true,
    };

    let result = formatter.fmt_repetition(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "{1,5}");
}



