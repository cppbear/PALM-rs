// Answer 0

#[test]
fn test_fmt_repetition_range_not_greedy() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ast::RepetitionKind::ZeroOrOne if ast.greedy => self.wtr.write_str("?"),
                ast::RepetitionKind::ZeroOrOne => self.wtr.write_str("??"),
                ast::RepetitionKind::ZeroOrMore if ast.greedy => self.wtr.write_str("*"),
                ast::RepetitionKind::ZeroOrMore => self.wtr.write_str("*?"),
                ast::RepetitionKind::OneOrMore if ast.greedy => self.wtr.write_str("+"),
                ast::RepetitionKind::OneOrMore => self.wtr.write_str("+?"),
                ast::RepetitionKind::Range(ref x) => {
                    self.fmt_repetition_range(x)?;
                    if !ast.greedy {
                        self.wtr.write_str("?")?;
                    }
                    Ok(())
                }
            }
        }

        fn fmt_repetition_range(&mut self, _x: &ast::RepetitionRange) -> fmt::Result {
            self.wtr.write_str("{1,3}") // Mocking the formatting of a repetition range
        }
    }

    let mut writer = MockWriter::new();
    let mut formatter = MockFormatter { wtr: &mut writer };

    let ast = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange {}) },
        greedy: false,
    };

    let result = formatter.fmt_repetition(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "{1,3}?");
}

