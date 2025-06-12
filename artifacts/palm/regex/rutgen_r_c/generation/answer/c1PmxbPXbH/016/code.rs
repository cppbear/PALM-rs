// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question_mark() {
    struct TestParser<'s> {
        position: Position,
        pattern: &'s str,
        char: char,
        asts: Vec<Ast>,
    }

    impl<'s> TestParser<'s> {
        fn new(pattern: &'s str, asts: Vec<Ast>, char: char) -> Self {
            TestParser {
                position: Position { offset: 0, line: 1, column: 1 },
                pattern,
                char,
                asts,
            }
        }

        fn bump(&mut self) {
            self.position.offset += 1;
            self.char = match self.pattern.chars().nth(self.position.offset) {
                Some(c) => c,
                None => '\0',
            };
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn error(&self, span: Span, error_kind: ast::ErrorKind) -> Error {
            Error {
                kind: error_kind,
                pattern: self.pattern.to_string(),
                span,
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn parse_uncounted_repetition(
            &self,
            mut concat: ast::Concat,
            kind: ast::RepetitionKind,
        ) -> Result<ast::Concat> {
            assert!(self.char == '?');
            let op_start = self.pos();
            let ast = match concat.asts.pop() {
                Some(ast) => ast,
                None => return Err(self.error(
                    self.span(),
                    ast::ErrorKind::RepetitionMissing,
                )),
            };
            match ast {
                Ast::Empty(_) | Ast::Flags(_) => return Err(self.error(
                    self.span(),
                    ast::ErrorKind::RepetitionMissing,
                )),
                _ => {}
            }
            concat.asts.push(Ast::Repetition(ast::Repetition {
                span: ast.span().with_end(self.pos()),
                op: ast::RepetitionOp {
                    span: Span::new(op_start, self.pos()),
                    kind,
                },
                greedy: true,
                ast: Box::new(ast),
            }));
            Ok(concat)
        }
    }

    let parser = TestParser::new("a?b", vec![Ast::Flags(ast::Flags { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) })], '?');
    let concat = ast::Concat { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 }), asts: vec![Ast::Flags(ast::Flags { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) })] };
    
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrOne);
    assert!(result.is_err());
}

#[test]
fn test_parse_uncounted_repetition_with_star() {
    struct TestParser<'s> {
        position: Position,
        pattern: &'s str,
        char: char,
        asts: Vec<Ast>,
    }

    impl<'s> TestParser<'s> {
        fn new(pattern: &'s str, asts: Vec<Ast>, char: char) -> Self {
            TestParser {
                position: Position { offset: 0, line: 1, column: 1 },
                pattern,
                char,
                asts,
            }
        }

        fn bump(&mut self) {
            self.position.offset += 1;
            self.char = match self.pattern.chars().nth(self.position.offset) {
                Some(c) => c,
                None => '\0',
            };
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn error(&self, span: Span, error_kind: ast::ErrorKind) -> Error {
            Error {
                kind: error_kind,
                pattern: self.pattern.to_string(),
                span,
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn parse_uncounted_repetition(
            &self,
            mut concat: ast::Concat,
            kind: ast::RepetitionKind,
        ) -> Result<ast::Concat> {
            assert!(self.char == '*');
            let op_start = self.pos();
            let ast = match concat.asts.pop() {
                Some(ast) => ast,
                None => return Err(self.error(
                    self.span(),
                    ast::ErrorKind::RepetitionMissing,
                )),
            };
            match ast {
                Ast::Empty(_) | Ast::Flags(_) => return Err(self.error(
                    self.span(),
                    ast::ErrorKind::RepetitionMissing,
                )),
                _ => {}
            }
            concat.asts.push(Ast::Repetition(ast::Repetition {
                span: ast.span().with_end(self.pos()),
                op: ast::RepetitionOp {
                    span: Span::new(op_start, self.pos()),
                    kind,
                },
                greedy: true,
                ast: Box::new(ast),
            }));
            Ok(concat)
        }
    }

    let parser = TestParser::new("a*b", vec![Ast::Flags(ast::Flags { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) })], '*');
    let concat = ast::Concat { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 }), asts: vec![Ast::Flags(ast::Flags { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) })] };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_err());
}

