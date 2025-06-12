// Answer 0

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
    struct MockWtr {
        output: String,
    }

    impl MockWtr {
        fn new() -> Self {
            MockWtr { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWtr,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { wtr: MockWtr::new() }
        }

        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ast::RepetitionKind::ZeroOrOne if ast.greedy => self.wtr.write_str("?"),
                ast::RepetitionKind::ZeroOrOne => self.wtr.write_str("??"),
                // other cases omitted for brevity
                _ => Ok(()),
            }
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: true,
    };
    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "?");
}

#[test]
fn test_fmt_repetition_zero_or_one_nongreedy() {
    struct MockWtr {
        output: String,
    }

    impl MockWtr {
        fn new() -> Self {
            MockWtr { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWtr,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { wtr: MockWtr::new() }
        }

        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ast::RepetitionKind::ZeroOrOne if ast.greedy => self.wtr.write_str("?"),
                ast::RepetitionKind::ZeroOrOne => self.wtr.write_str("??"),
                // other cases omitted for brevity
                _ => Ok(()),
            }
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: false,
    };
    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "??");
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    struct MockWtr {
        output: String,
    }

    impl MockWtr {
        fn new() -> Self {
            MockWtr { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWtr,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { wtr: MockWtr::new() }
        }

        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ast::RepetitionKind::ZeroOrMore if ast.greedy => self.wtr.write_str("*"),
                ast::RepetitionKind::ZeroOrMore => self.wtr.write_str("*?"),
                // other cases omitted for brevity
                _ => Ok(()),
            }
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
    };
    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "*");
}

#[test]
fn test_fmt_repetition_zero_or_more_nongreedy() {
    struct MockWtr {
        output: String,
    }

    impl MockWtr {
        fn new() -> Self {
            MockWtr { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWtr,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { wtr: MockWtr::new() }
        }

        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ast::RepetitionKind::ZeroOrMore if ast.greedy => self.wtr.write_str("*"),
                ast::RepetitionKind::ZeroOrMore => self.wtr.write_str("*?"),
                // other cases omitted for brevity
                _ => Ok(()),
            }
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: false,
    };
    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "*?");
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    struct MockWtr {
        output: String,
    }

    impl MockWtr {
        fn new() -> Self {
            MockWtr { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWtr,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { wtr: MockWtr::new() }
        }

        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ast::RepetitionKind::OneOrMore if ast.greedy => self.wtr.write_str("+"),
                ast::RepetitionKind::OneOrMore => self.wtr.write_str("+?"),
                // other cases omitted for brevity
                _ => Ok(()),
            }
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
    };
    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "+");
}

#[test]
fn test_fmt_repetition_one_or_more_nongreedy() {
    struct MockWtr {
        output: String,
    }

    impl MockWtr {
        fn new() -> Self {
            MockWtr { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWtr,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { wtr: MockWtr::new() }
        }

        fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ast::RepetitionKind::OneOrMore if ast.greedy => self.wtr.write_str("+"),
                ast::RepetitionKind::OneOrMore => self.wtr.write_str("+?"),
                // other cases omitted for brevity
                _ => Ok(()),
            }
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
    };
    formatter.fmt_repetition(&ast).unwrap();
    assert_eq!(formatter.wtr.output, "+?");
}

